use crate::sols_cf::functions::b_print_factors_1::is_factor;

pub fn is_prime(n: i32) -> bool {
    let mut ctr = 0;
    for i in 1..=n {
        if is_factor(n, i) {
            ctr += 1;
        }
    }

    ctr == 2
}

pub fn write_primes(n: i32) {
    for i in 2..=n {
        if is_prime(i) {
            print!("{} ", i);
        }
    }
}
