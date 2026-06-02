use crate::sols_cf::functions::e_factorial::get_factorial;

pub fn get_n_c_r(n: i32, m: i32) -> i64 {
    get_factorial(n) / (get_factorial(m) * get_factorial(n - m))
}
