pub fn write_vertical_triangle(n: i32) {
    let mut m = 1;
    let mut flipper = true;
    for _ in 0..((2 * n) - 1) {
        for _ in 0..m {
            print!("*");
        }
        if m == n {
            flipper = false;
        }
        if flipper {
            m += 1;
        } else {
            m -= 1;
        }

        println!("");
    }
}
