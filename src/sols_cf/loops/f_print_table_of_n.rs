fn print_table_of_n(n: i32) {
    for i in 1..=10 {
        println!("{} * {} = {}", n, i, n * i);
    }
}
fn main() {
    print_table_of_n(6);
}
