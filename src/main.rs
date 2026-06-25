use crate::arrays_and_slices::slices;
use crate::literals::literal;
use crate::primitives::primitiv;
use crate::tuples::tupli;

pub mod primitives;
pub mod literals;
pub mod tuples;
pub mod arrays_and_slices;

fn main() {
    primitiv();
    literal();
    tupli();
    slices()
}
