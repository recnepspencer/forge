//! Independent vendored expansion arithmetic.

mod fast_sum;
mod fixed_product;
mod fixed_sum;
mod growth;
mod primitives;
mod scale;
mod sum;

pub(in crate::predicates) use fast_sum::fast_expansion_sum_zeroelim;
pub(in crate::predicates) use fixed_product::{
    four_one_product, two_one_product, two_square, two_two_product,
};
pub(in crate::predicates) use fixed_sum::{
    eight_four_sum, eight_one_sum, eight_two_sum, four_four_sum, four_one_sum, four_two_sum,
    two_one_diff, two_one_sum, two_two_diff, two_two_sum,
};
pub(in crate::predicates) use growth::{grow_expansion, grow_expansion_zeroelim};
pub(in crate::predicates) use primitives::{
    fast_two_diff, fast_two_diff_tail, fast_two_sum, fast_two_sum_tail, split, square, square_tail,
    two_diff, two_diff_tail, two_product, two_product_2presplit, two_product_presplit,
    two_product_tail, two_sum, two_sum_tail,
};
pub(in crate::predicates) use scale::scale_expansion_zeroelim;
pub(in crate::predicates) use sum::{
    expansion_sum, expansion_sum_zeroelim1, expansion_sum_zeroelim2,
};
