pub fn to_the_power(x: i32, n: i32) -> i64 {
    let mut total: i64 = 1;

    for _ in 1..=n {
        total *= x as i64;
    }

    return total;
}
