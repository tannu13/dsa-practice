pub fn write_hollow_rectangle(n: i32, m: i32) {
    for i in 1..=n {
        for j in 1..=m {
            if i == 1 || i == n || j == 1 || j == m {
                print!("*");
            } else {
                print!(" ");
            }
        }
        println!(" ");
    }
}
