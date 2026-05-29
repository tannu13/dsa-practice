pub fn write_hollow_diamond(n: i32) {
    let mut m = 1;
    let mut spacer;
    let mut reached_midway = false;

    for i in 1..=((2 * n) - 1) {
        if !reached_midway {
            spacer = n - i;
        } else {
            spacer = i - n;
        }

        for _ in 0..spacer {
            print!(" ");
        }

        for j in 1..=m {
            if i == 1 || i == ((2 * n) - 1) || j == 1 || j == m {
                print!("* ");
            } else {
                print!("  ");
            }
        }
        println!("");

        if i == n {
            reached_midway = true;
        }

        if !reached_midway {
            m += 1;
        } else {
            m -= 1;
        }
    }
}
