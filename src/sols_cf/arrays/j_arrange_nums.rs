pub fn arrange_nums<const N: usize>() {
    let mut i = 0;
    let mut j = N - 1;
    let mut ctr = 1;
    let mut m = [0; N];
    while i <= j {
        m[j] = ctr + 1;
        m[i] = ctr;

        ctr += 2;
        i += 1;
        j -= 1;
    }

    println!("{:?}", m);
}
