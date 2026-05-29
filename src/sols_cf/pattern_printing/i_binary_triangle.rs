pub fn write_binary_triangle(n: i32) {
    let mut m = 0;
    for i in 0..n {
        let mut counter = i;
        for _ in 0..=m {
            print!("{}", counter % 2);
            counter += 1;
        }
        m += 1;
        println!("");
    }
}
