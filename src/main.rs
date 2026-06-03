use crate::sols_cf::arrays::{
    a_print_array_in_reverse::write_array_in_reverse, b_sum_of_array::sum_of_array,
    c_min_element_n_position::write_min_elem_n_pos, d_max_element_n_position::write_max_elem_n_pos,
    e_search_elem_in_array::search_elem_in_array, f_count_occurrences::count_occurrences,
};

mod sols_cf;

fn main() {
    let arr = [7, 3, 4, 5, 3, 10];
    count_occurrences(arr, 101);
}
