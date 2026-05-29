pub fn sum_of_digits(mut n: i64) {
    let mut total = 0;

    while n > 0 {
        total += n % 10;
        n /= 10;
    }

    println!("{}", total);
}
