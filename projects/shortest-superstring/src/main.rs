use std::time::Instant;
use superstring::{short_super_string, shortest_super_string};

fn main() {
    let days = &days_in_year(true)[0..14];
    let start = Instant::now();
    let superstring = shortest_super_string(days);
    println!("{}", superstring);
    println!("{}ms", start.elapsed().as_millis());
    let start = Instant::now();
    let superstring = short_super_string(days);
    println!("{}", superstring);
    println!("{}ms", start.elapsed().as_millis());
}

// 0120118011701160115011401130112011101101090108010701060105010401030102010119

pub fn days_in_year(leap_year: bool) -> Vec<String> {
    let days_in_month = vec![31, if leap_year { 29 } else { 28 }, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31];
    let mut days = Vec::new();
    for (month, &days_in_month) in days_in_month.iter().enumerate() {
        for day in 1..=days_in_month {
            days.push(format!("{:02}{:02}", month + 1, day));
        }
    }
    days
}
