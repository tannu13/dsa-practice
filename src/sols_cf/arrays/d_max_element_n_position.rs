pub fn write_max_elem_n_pos<const N: usize>(m: [i32; N]) {
    let mut max = m[0];
    let mut pos: usize = 0;
    for i in 1..N {
        if max < m[i] {
            max = m[i];
            pos = i + 1;
        }
    }

    println!("{} {}", max, pos);
}
