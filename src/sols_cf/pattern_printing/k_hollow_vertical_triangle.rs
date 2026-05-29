pub fn write_hollow_vertical_triangle(n: i32) {
    let mut m = 1;
    let mut flipper = true;
    for i in 0..((2 * n) - 1) {
        for j in 1..=m {
            if i == 0 || j == 1 || i == ((2 * n) - 1) || j == m {
                print!("* ");
            } else {
                print!(" ");
            }
        }
        println!("");
        if m == n {
            flipper = false;
        }
        if flipper {
            m += 1;
        } else {
            m -= 1;
        }
    }
}
