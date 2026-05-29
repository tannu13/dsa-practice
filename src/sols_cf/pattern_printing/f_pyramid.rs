pub fn write_pyramid(n: i32) {
    let mut m = 1;
    for _ in 1..=n {
        for _ in 1..=m {
            print!("*");
        }
        m += 1;
        println!("");
    }
}
