pub fn write_inverse_pyramid(n: i32) {
    let mut m = n;
    for _ in 1..=n {
        for _ in 1..=m {
            print!("*");
        }
        println!("");
        m -= 1;
    }
}
