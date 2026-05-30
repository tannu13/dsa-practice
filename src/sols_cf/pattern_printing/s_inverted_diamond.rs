pub fn write_inverted_diamond(n: u32) {
    for i in 1..=(2 * n) - 1 {
        let (stars, spaces) = if i <= n {
            (n - i + 1, 2 * (i - 1) + 1)
        } else {
            (i - n + 1, (4 * n) - (2 * i) - 1)
        };

        for _ in 0..stars {
            print!("*");
        }

        for _ in 0..spaces {
            print!(" ");
        }

        for _ in 0..stars {
            print!("*");
        }
        println!();
    }
}
