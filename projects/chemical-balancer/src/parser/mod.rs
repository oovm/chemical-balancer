use crate::{ChemicalBalancer, ChemicalKind, ChemicalTerm};
use pex::{
    helpers::{decimal_string, make_from_str, whitespace},
    ParseResult, ParseState, StopBecause,
};
use std::str::FromStr;

impl FromStr for ChemicalBalancer {
    type Err = StopBecause;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let state = ParseState::new(s.trim_end()).skip(whitespace);
        make_from_str(state, Self::parse)
    }
}

impl FromStr for ChemicalTerm {
    type Err = StopBecause;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let state = ParseState::new(s.trim_end()).skip(whitespace);
        make_from_str(state, Self::parse)
    }
}

impl ChemicalBalancer {
    pub fn parse(state: ParseState) -> ParseResult<Self> {
        let (state, mut equation) = ChemicalBalancer::parse_add(state)?;
        let (state, _) = state.match_fn(Self::parse_eq)?;
        let (state, rhs) = ChemicalBalancer::parse_add(state.skip(whitespace))?;
        equation.extend(rhs);
        let mut out = ChemicalBalancer { elements: Default::default(), equation };
        out.record_elements();

        state.finish(out)
    }
    fn parse_add(state: ParseState) -> ParseResult<Vec<ChemicalTerm>> {
        let (state, first) = ChemicalTerm::parse(state)?;
        let (state, rest) = state.match_repeats(|s| {
            let (s, _) = s.skip(whitespace).match_char('+')?;
            ChemicalTerm::parse(s.skip(whitespace))
        })?;
        let mut out = vec![first];
        out.extend(rest);
        state.finish(out)
    }
    pub fn parse_eq(state: ParseState) -> ParseResult<String> {
        let (state, eq) = state
            .skip(whitespace)
            .begin_choice()
            .or_else(|s| s.match_str("="))
            .or_else(|s| s.match_str("=="))
            .or_else(|s| s.match_str("=>"))
            .or_else(|s| s.match_str("->"))
            .end_choice()?;
        state.finish(eq.to_string())
    }
}

impl ChemicalTerm {
    pub fn parse(state: ParseState) -> ParseResult<Self> {
        let (state, first) = state.match_repeat_m_n(1, 255, Self::parse_term)?;
        state.finish(Self::compound(first))
    }
    fn parse_term(state: ParseState) -> ParseResult<Self> {
        let (state, mut term) = state
            .skip(whitespace)
            .begin_choice()
            .or_else(|s| Self::parse_paired(s, '(', ')'))
            .or_else(|s| Self::parse_paired(s, '[', ']'))
            .or_else(|s| Self::parse_paired(s, '{', '}'))
            .or_else(Self::parse_atom)
            .end_choice()?;
        // let (state, n) = state.match_optional(Self::parse_electronic)?;
        // term.set_electronic(n.unwrap_or(1.0));
        let (state, n) = state.match_optional(Self::parse_number)?;
        term.set_number(n.unwrap_or(1.0));
        state.finish(term)
    }
    // _? number
    fn parse_number(state: ParseState) -> ParseResult<f64> {
        let (state, _) = state.skip(whitespace).match_optional(|s| s.match_char('_'))?;
        let (state, number) = state.skip(whitespace).match_fn(parse_decimal)?;
        state.finish(number)
    }

    pub fn parse_electronic(state: ParseState) -> ParseResult<f64> {
        let (state, _) = state.skip(whitespace).match_optional(|s| s.match_char('^'))?;
        let (state, number) = state.skip(whitespace).match_fn(parse_decimal)?;
        state.finish(number)
    }

    fn parse_paired(state: ParseState, start: char, end: char) -> ParseResult<ChemicalTerm> {
        let kind = ChemicalKind::Paired(start, end);
        let (state, _) = state.match_char(start)?;
        let (state, compound) = state.skip(whitespace).match_repeat_m_n(1, 255, Self::parse)?;
        // println!("Nested {:?}", cs);
        let (state, _) = state.skip(whitespace).match_char(end)?;
        state.finish(ChemicalTerm { kind, compound, count: 1.0, electronic: 0.0 })
    }

    // uppercase letter + lowercase letters
    // eg: C, H, O, Na, Cl, Uuq, Ph(benzene), Benzene
    fn parse_atom(state: ParseState) -> ParseResult<ChemicalTerm> {
        let (state, head) = state.match_char_if(|c| c.is_ascii_uppercase(), "Uppercase letter")?;
        let (state, tail) = state.match_repeats(|s| s.match_char_if(|c| c.is_ascii_lowercase(), "Lowercase letters"))?;
        let mut atom = head.to_string();
        for c in tail {
            atom.push(c);
        }
        state.finish(ChemicalTerm::atom(atom))
    }
}

// decimal = integer (. integer)?
fn parse_decimal(state: ParseState) -> ParseResult<f64> {
    let (state, a) = decimal_string(state)?;
    let a = f64::from_str(a)?;
    state.finish(a)
}
