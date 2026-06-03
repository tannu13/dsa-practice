pub fn search_elem_in_array<const N: usize>(m: [i32; N], target: i32) {
    let mut found = false;
    for i in 0..N {
        if target == m[i] {
            found = true;
            break;
        }
    }

    if found {
        print!("YES");
    } else {
        print!("NO");
    }
}
