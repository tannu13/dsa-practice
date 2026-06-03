pub fn write_array_in_reverse<const N: usize>(arr: [i32; N]) {
    for i in (0..N).rev() {
        print!("{} ", arr[i]);
    }
}
