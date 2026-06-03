pub fn write_min_elem_n_pos<const N: usize>(m: [i32; N]) {
    let mut min = m[0];
    let mut pos: usize = 0;
    for i in 1..N {
        if min > m[i] {
            min = m[i];
            pos = i + 1;
        }
    }

    println!("{} {}", min, pos);
}
