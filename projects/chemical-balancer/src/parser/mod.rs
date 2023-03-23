use std::str::FromStr;
use pex::{ParseResult, ParseState, StopBecause};
use crate::Compound;

impl FromStr for Compound {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        todo!()
        // ParseState::new(s)
    }
}

impl Compound {
    pub fn parse(state: ParseState) -> ParseResult<Self> {
        let (state, compound) = state.match_repeats(|s|
            {
                let (state, _) = s.match_optional(parse_whitespace)?;
                state.begin_choice()
                    .maybe(Self::parse_parentheses_count)
                    .maybe(Self::parse_atom_count)
                    .end_choice()
            }
        )?;
        state.skip(parse_whitespace);
        let (state, number) = state.match_optional(parse_decimal)?;
        state.finish(Compound::Compound {
            group: Default::default(),
            compound,
            count: number.unwrap_or(1.0),
        })
    }


    fn parse_parentheses_count(state: ParseState) -> ParseResult<Compound> {
        let (state, _) = state.match_char('(')?;
        state.skip(parse_whitespace);
        let (state, cs) = state.match_repeats(Self::parse)?;
        state.skip(parse_whitespace);
        let (state, _) = state.match_char(')')?;
        state.skip(parse_whitespace);
        let (state, number) = state.match_optional(parse_decimal)?;
        state.finish(Compound::parentheses(cs, number.unwrap_or(1.0)))
    }


    fn parse_atom_count(state: ParseState) -> ParseResult<Compound> {
        let (state, atom) = Self::parse_atom(state)?;
        state.skip(parse_whitespace);
        let (state, number) = state.match_optional(parse_decimal)?;
        println!("{} {:?}", atom, number);
        state.finish(Compound::Atom {
            atom,
            count: number.unwrap_or(1.0),
        })
    }

    // uppercase letter + lowercase letters
    // eg: C, H, O, Na, Cl, Uuq, Ph(benzene), Benzene
    fn parse_atom(state: ParseState) -> ParseResult<String> {
        let (state, head) = state.match_char_if(|c| c.is_ascii_uppercase(), "Uppercase letter")?;
        let (state, tail) = state.match_repeats(|s| s.match_char_if(|c| c.is_ascii_lowercase(), "Lowercase letters"))?;
        let mut atom = head.to_string();
        for c in tail {
            atom.push(c);
        }
        state.finish(atom)
    }
}

fn parse_whitespace(state: ParseState) -> ParseResult<()> {
    match state.rest_text.find(|c: char| !c.is_ascii_whitespace()) {
        None => {
            state.finish(())
        }
        Some(s) => {
            state.advance(s).finish(())
        }
    }
}

// decimal = integer (. integer)?
fn parse_decimal(state: ParseState) -> ParseResult<f64> {
    let (state, integer) = parse_integer(state)?;
    let (state, rest) = state.match_optional(|s| {
        let (s, _) = s.match_char('.')?;
        parse_integer(s)
    })?;
    match rest {
        // usize
        Some(s) => {
            state.finish(build_f64(integer, s))
        }
        None => {
            state.finish(integer as f64)
        }
    }
}

fn build_f64(integer: usize, decimal: usize) -> f64 {
    let mut out = integer as f64;
    let mut decimal = decimal as f64;
    while decimal > 0.0 {
        decimal /= 10.0;
        out += decimal;
    }
    out
}


// integer = (_? + digit)+
fn parse_integer(state: ParseState) -> ParseResult<usize> {
    let mut offset = 0;
    let mut has_number = false;
    for c in state.rest_text.chars() {
        match c {
            '_' => {
                offset += 1;
            }
            '0'..='9' => {
                has_number = true;
                offset += 1;
            }
            _ => {
                break;
            }
        }
    }
    if offset.eq(&0) || !has_number {
        Err(StopBecause::MissingString { message: "Except digits", position: state.start_offset })?
    } else {
        let integer = state.rest_text[..offset].parse().unwrap();
        state.advance(offset).finish(integer)
    }

}



#[test]
fn test() {
    let co2 = ParseState::new("S2(CO3)");
    let co2 = Compound::parse(co2);
    println!("{:#?}", co2)
}