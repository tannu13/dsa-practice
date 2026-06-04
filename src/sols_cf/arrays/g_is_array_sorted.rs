pub fn is_array_sorted<const N: usize>(m: [i32; N]) {
    let mut is_sorted = true;
    for i in 1..N {
        if m[i] < m[i - 1] {
            is_sorted = false;
        }
    }

    if is_sorted {
        print!("YES");
    } else {
        print!("NO");
    }
}
