pub fn sort_01<const N: usize>(m: [i32; N]) {
    let mut c0 = 0;
    let mut c1 = 0;

    for i in 0..N {
        if m[i] == 0 {
            c0 += 1;
        } else {
            c1 += 1;
        }
    }

    for _ in 0..c0 {
        print!("0 ");
    }
    for _ in 0..c1 {
        print!("1 ");
    }
}
