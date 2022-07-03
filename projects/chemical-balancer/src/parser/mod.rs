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
    pub fn parse(state: ParseState) -> Result<Self, ()> {
        todo!()
    }

    // uppercase letter + lowercase letters
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

#[test]
fn test() {
    let co2 = ParseState::new("CO2");
    let co2 = Compound::parse_atom(co2);
    println!("{:?}", co2)
}