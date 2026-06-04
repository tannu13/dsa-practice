pub fn reverse_array_in_place<const N: usize>(mut m: [i32; N]) {
    let mut i = 0;
    let mut j = N - 1;
    while i < j {
        let temp = m[i];
        m[i] = m[j];
        m[j] = temp;

        i += 1;
        j -= 1;
    }

    println!("{:?}", m);
}
