pub fn write_triangle(n: i32) {
    let mut m = 1;
    let mut spacer: i32;

    for i in 0..n {
        spacer = n - i;
        for _ in 0..spacer {
            print!(" ");
        }

        for j in 1..=m {
            if j == m {
                print!("*");
            } else {
                print!("* ");
            }
        }
        m += 1;
        println!("");
    }
}
