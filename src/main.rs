use crate::sols_cf::functions::{
    a_hello_functions::write_hello_fns, b_print_factors_1::is_factor,
    c_print_factors_2::is_factor_2, d_check_prime::is_prime, e_factorial::get_factorial,
    f_binomial_coefficient::get_n_c_r, g_print_primes_upto_n::write_primes,
    h_count_zeros::count_zeros, i_find_hcf::find_hcf,
};

mod sols_cf;

fn main() {
    let n = 12;
    let m = 36;
    println!("{} ", find_hcf(n, m));
}
