pub fn write_crown(n: u32) {
    for i in 1..=n {
        let spacer_len = (2 * n) - (2 * i);
        let mut spaces_written = 0;
        for j in 1..=2 * n {
            if j <= i {
                print!("*");
            } else if spaces_written < spacer_len {
                print!(" ");
                spaces_written += 1;
            } else {
                print!("*");
            }
        }
        println!("");
    }
}
