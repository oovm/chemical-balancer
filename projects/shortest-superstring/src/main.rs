fn main() {
    let days = &days_in_year(true)[0..10];
    let start = Instant::now();
    let superstring = Solution::shortest_superstring(days);
    println!("{}", superstring);
    println!("{}ms", start.elapsed().as_millis());

    let start = Instant::now();
    let superstring = shortest_super_string(days);
    println!("{}", superstring);
    println!("{}ms", start.elapsed().as_millis());
}
