pub fn sum_of_array<const N: usize>(m: [i32; N]) {
    let mut sum = 0;
    for i in 0..N {
        sum += m[i];
    }

    println!("{}", sum);
}
