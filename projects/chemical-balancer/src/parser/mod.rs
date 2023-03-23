use crate::{ChemicalBalancer, Compound};
use pex::{ParseResult, ParseState, StopBecause};
use std::str::FromStr;

impl FromStr for ChemicalBalancer {
    type Err = StopBecause;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let state = ParseState::new(s.trim_end()).skip(parse_whitespace);
        match Self::parse(state) {
            ParseResult::Pending(state, compound) if state.is_empty() => Ok(compound),
            ParseResult::Pending(state, ..) => Err(StopBecause::ExpectEof { position: state.start_offset }),
            ParseResult::Stop(e) => Err(e),
        }
    }
}

impl FromStr for Compound {
    type Err = StopBecause;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let state = ParseState::new(s.trim_end()).skip(parse_whitespace);
        match Self::parse(state) {
            ParseResult::Pending(state, compound) if state.is_empty() => Ok(compound),
            ParseResult::Pending(state, ..) => Err(StopBecause::ExpectEof { position: state.start_offset }),
            ParseResult::Stop(e) => Err(e),
        }
    }
}

impl ChemicalBalancer {
    pub fn parse(state: ParseState) -> ParseResult<Self> {
        let (state, lhs) = ChemicalBalancer::parse_add(state)?;
        let (state, _) = state.skip(parse_whitespace).match_char('=')?;
        let (state, rhs) = ChemicalBalancer::parse_add(state.skip(parse_whitespace))?;

        let mut out = ChemicalBalancer { elements: Default::default(), lhs, rhs };
        out.record_elements();

        state.finish(out)
    }
    fn parse_add(state: ParseState) -> ParseResult<Vec<Compound>> {
        let (state, first) = Compound::parse(state)?;
        let (state, rest) = state.match_repeats(|s| {
            let (s, _) = s.skip(parse_whitespace).match_char('+')?;
            Compound::parse(s.skip(parse_whitespace))
        })?;
        let mut out = vec![first];
        out.extend(rest);
        state.finish(out)
    }
}

impl Compound {
    pub fn parse(state: ParseState) -> ParseResult<Self> {
        let (state, compound) = state.match_repeat_m_n(1, 255, |s| {
            s.skip(parse_whitespace)
                .begin_choice()
                .maybe(Self::parse_atom_count)
                .maybe(Self::parse_parentheses_count)
                .end_choice()
        })?;
        let (state, number) = state.skip(parse_whitespace).match_optional(parse_decimal)?;
        state.finish(Compound::compound(compound, number.unwrap_or(1.0)))
    }

    fn parse_parentheses_count(state: ParseState) -> ParseResult<Compound> {
        let (state, _) = state.match_char('(')?;
        let (state, cs) = state.skip(parse_whitespace).match_repeat_m_n(1, 255, Self::parse)?;
        println!("Nested {:?}", cs);
        let (state, _) = state.skip(parse_whitespace).match_char(')')?;
        let (state, number) = state.skip(parse_whitespace).match_optional(parse_decimal)?;
        state.finish(Compound::parentheses(cs, number.unwrap_or(1.0)))
    }

    fn parse_atom_count(state: ParseState) -> ParseResult<Compound> {
        let (state, atom) = Self::parse_atom(state)?;
        let (state, number) = state.skip(parse_whitespace).match_optional(parse_decimal)?;
        // println!("{} {:?}", atom, number);
        state.finish(Compound::atom(atom, number.unwrap_or(1.0)))
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
        None => state.finish(()),
        Some(s) => state.advance(s).finish(()),
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
        Some(s) => state.finish(build_f64(integer, s)),
        None => state.finish(integer as f64),
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
    }
    else {
        let integer = state.rest_text[..offset].parse().unwrap();
        state.advance(offset).finish(integer)
    }
}
