fn main() {
    let n = 5;
    let nums = vec![-2, 0, 3, 7, -5];

    let mut positive_nums = 0;
    let mut negative_nums = 0;
    let mut even_nums = 0;
    let mut odd_nums = 0;

    for i in 0..n {
        let num = nums[i];
        if num > 0 {
            positive_nums += 1;
        }
        if num < 0 {
            negative_nums += 1;
        }

        if num % 2 == 0 {
            even_nums += 1;
        } else {
            odd_nums += 1;
        }
    }

    println!("{}", positive_nums);
    println!("{}", negative_nums);
    println!("{}", even_nums);
    println!("{}", odd_nums);
}
