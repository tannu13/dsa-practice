pub fn reverse_num_n_store_in_var(mut n: i32) {
    let mut rev: i32 = 0;
    while n > 0 {
        rev = (rev * 10) + (n % 10);
        n = n / 10;
    }

    println!("{}", rev);
}
