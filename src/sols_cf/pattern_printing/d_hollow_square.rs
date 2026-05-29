pub fn write_hollow_square(n: i32) {
    for i in 1..=n {
        for j in 1..=n {
            if i == 1 || i == n || j == 1 || j == n {
                print!("*");
            } else {
                print!(" ");
            }
        }
        println!(" ");
    }
}
