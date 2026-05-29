pub fn write_hollow_triangle(n: i32) {
    let mut m = 1;
    let mut spacer;

    for i in 0..n {
        spacer = n - i;
        for _ in 0..spacer {
            print!(" ");
        }
        for j in 1..=m {
            if i == 0 || i == n - 1 || j == 1 || j == m {
                print!("* ");
            } else {
                print!("  ");
            }
        }
        println!("");
        m += 1;
    }
}
