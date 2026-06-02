pub fn get_factorial(n: i32) -> i64 {
    let mut factorial: i64 = 1;
    for i in 1..=n {
        factorial *= i as i64;
    }

    factorial
}
