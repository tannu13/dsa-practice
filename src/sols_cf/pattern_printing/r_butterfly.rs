pub fn write_butterfly(n: u32) {
    let mut reached_midway = false;

    for i in 1..=(2 * n) - 1 {
        let spacer_len;
        let mut spaces_written = 0;
        if !reached_midway {
            spacer_len = 2 * n - 2 * i;
        } else {
            spacer_len = 2 * i - 2 * n;
        }

        for j in 1..=(2 * n) {
            if !reached_midway && j <= i {
                print!("*");
            } else if reached_midway && j <= i - spacer_len {
                print!("*");
            } else if spaces_written < spacer_len {
                print!(" ");
                spaces_written += 1;
            } else {
                print!("*");
            }
        }

        if n == i {
            reached_midway = true;
        }
        println!("");
    }
}
