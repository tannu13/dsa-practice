use crate::sols_cf::pattern_printing::{
    g_inverse_pyramid::write_inverse_pyramid, h_numbered_triangle::write_numbered_triangle,
    i_binary_triangle::write_binary_triangle, j_vertical_triangle::write_vertical_triangle,
    k_hollow_vertical_triangle::write_hollow_vertical_triangle, l_triangle::write_triangle,
    m_hollow_triangle::write_hollow_triangle,
    n_inverted_hollow_triangle::write_inverted_hollow_triangle, o_diamond::write_diamond,
    p_hollow_diamond::write_hollow_diamond,
};

mod sols_cf;

fn main() {
    write_hollow_diamond(4);
}
