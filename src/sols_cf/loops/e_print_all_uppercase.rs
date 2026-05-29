// Print All Uppercase Alphabets
fn print_all_uppercase_alphabets() {
    let mut x = b'A';
    for _ in 1..=26 {
        let next_char = (x) as char;
        print!("{} ", next_char);
        x = x + 1;
    }
}

fn main() {
    print_all_uppercase_alphabets();
}
