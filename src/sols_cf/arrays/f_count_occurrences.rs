pub fn count_occurrences<const N: usize>(m: [i32; N], target: i32) {
    let mut ctr = 0;
    for i in 0..N {
        if target == m[i] {
            ctr += 1;
        }
    }
    print!("{}", ctr);
}
