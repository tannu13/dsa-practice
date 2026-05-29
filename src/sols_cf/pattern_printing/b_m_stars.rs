pub fn write_m_stars(n: i32, m: i32) {
    for _ in 0..n {
        for _ in 0..m {
            print!("*");
        }
        println!("");
    }
}
