pub fn factorial() {
    let n = 20;

    let mut total: i64 = 1;

    for i in 1..=n {
        total *= i;
    }

    println!("{}", total);
}
