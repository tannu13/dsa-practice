use crate::sols_cf::arrays::{
    a_print_array_in_reverse::write_array_in_reverse, b_sum_of_array::sum_of_array,
    c_min_element_n_position::write_min_elem_n_pos, d_max_element_n_position::write_max_elem_n_pos,
    e_search_elem_in_array::search_elem_in_array, f_count_occurrences::count_occurrences,
    g_is_array_sorted::is_array_sorted, h_sort_01::sort_01, i_reverse::reverse_array_in_place,
    j_arrange_nums::arrange_nums,
};

mod sols_cf;

fn main() {
    let arr = [1, 2, 3, 4, 5];
    arrange_nums::<6>();
}
