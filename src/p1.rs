fn calc_fuel_req(m: i64) -> i64 {
    if m < 9 {
        0
    } else {
        m / 3 - 2
    }
}

fn calc_fuel_req_rec(m: i64) -> i64 {
    match calc_fuel_req(m) {
        0 => 0,
        f => f + calc_fuel_req_rec(f),
    }
}

fn main() {
    let masses = include_str!("p1.txt")
        .lines()
        .map(|line| {
            line.parse::<i64>()
                .expect(&format!("Failed to parse '{}'", line))
        })
        .collect::<Vec<_>>();

    // Part 1:
    println!(
        "Part 1: {}",
        masses
            .iter()
            .cloned()
            .map(calc_fuel_req)
            .fold(0, |acc, val| acc + val)
    );

    // Part 2:
    println!(
        "Part 2: {}",
        masses
            .iter()
            .cloned()
            .map(calc_fuel_req_rec)
            .fold(0, |acc, val| acc + val)
    );
}
