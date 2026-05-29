pub fn print_nums_in_reverse(mut n: i64) {
    if n == 0 {
        println!("{}", 0);
        return;
    }

    let mut nums: i64;

    while n > 0 {
        nums = n % 10;
        print!("{}", nums);
        n = n / 10;
    }
}
