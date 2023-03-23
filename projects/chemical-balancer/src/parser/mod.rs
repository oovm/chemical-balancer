use std::str::FromStr;
use pex::{ParseResult, ParseState};
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
        todo!()
    }

    pub fn parse_parentheses(state: ParseState) -> ParseResult<Compound> {
        let (state, _) = state.match_char('(')?;
        let (state, compound) = Self::parse(state)?;
        let (state, _) = state.match_char(')')?;
        state.finish(Compound::Compound {
            group: Default::default(),
            compound: vec![compound],
            count: 1.0,
        })
    }


    fn parse_atom_count(state: ParseState) -> ParseResult<Compound> {
        let (state, atom) = Self::parse_atom(state)?;
        let (state, _) = state.match_optional(parse_whitespace)?;
        let (state, number) = state.match_optional(parse_decimal)?;
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
    state.rest_text.find(|c: char| !c.is_ascii_whitespace()).map_or(state.finish(()), |i| state.advance(i).finish(()))
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
            let digits = (s as f64).log10().floor() as usize;
            state.finish(integer as f64 + s as f64 / 10.0f64.powi(digits as i32))
        }
        None => {
            state.finish(integer as f64)
        }
    }
}


// integer = (_? + digit)+
fn parse_integer(state: ParseState) -> ParseResult<usize> {
    let (state, integer) = state.match_repeats(parse_digit)?;
    let mut out = 0;
    for c in integer {
        out = out * 10 + (c as usize - '0' as usize);
    }
    state.finish(out)
}

// digit = _? + decimal_number
fn parse_digit(state: ParseState) -> ParseResult<char> {
    let (state, _) = state.match_optional(|s| s.match_char('_'))?;
    let (state, digit) = state.match_char_if(|c| c.is_ascii_digit(), "Digit")?;
    state.finish(digit)
}


#[test]
fn test() {
    let co2 = ParseState::new("CO _2");
    let co2 = Compound::parse_atom_count(co2);
    println!("{:?}", co2)
}