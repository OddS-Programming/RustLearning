use crate::declare::declare;
use crate::freezing::freezing;
use crate::mutable::mutanti;
use crate::scope::scopes;
use crate::shadowing::shadowing;
use crate::variables::variables;

pub mod variables;
pub mod mutable;
pub mod scope;
pub mod shadowing;
pub mod declare;
pub mod freezing;

fn main() {
    variables();
    mutanti();
    scopes();
    shadowing();
    declare();
    freezing();
}
