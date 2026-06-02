pub fn count_zeros(mut n: i32) -> i32 {
    let mut m;
    let mut ctr = 0;
    loop {
        m = n % 10;
        n = n / 10;

        if m == 0 {
            ctr += 1;
        }

        if n == 0 {
            break;
        }
    }

    ctr
}
