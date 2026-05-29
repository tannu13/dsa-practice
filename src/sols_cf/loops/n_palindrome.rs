pub fn palindrome(mut num: i32) {
    let input = num;
    let mut reverse = 0;

    while num > 0 {
        reverse = (reverse * 10) + (num % 10);
        num /= 10;
    }

    if input == reverse {
        println!("YES");
    } else {
        println!("NO");
    }
}
