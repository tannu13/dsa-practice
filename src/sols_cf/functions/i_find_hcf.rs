pub fn find_hcf(n: i32, m: i32) -> i32 {
    let mut starting_pos;
    let mut greater_num;
    if n > m {
        starting_pos = m;
        greater_num = n;
    } else {
        starting_pos = n;
        greater_num = m;
    }

    for i in (1..=starting_pos).rev() {
        if starting_pos % i == 0 && greater_num % 1 == 0 {
            return i;
        }
    }

    1
}
