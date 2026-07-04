//Примитивные типы могут быть сконвертированы в другие при помощи приведения типов.
//
// Rust предоставляет преобразование между пользовательскими типами (такими как, struct и enum) через использование трейтов.
// Общие преобразования используют трейты From и Into.
// Однако есть и конкретные трейты для более частных случаев, например для конвертации String.

use crate::from::froms;
use crate::fromstr_tostr::{fromstr, tostring};
use crate::into::intoos;
use crate::tryfrom_tryinto::trial;

pub mod from;
pub mod into;
pub mod tryfrom_tryinto;
pub mod fromstr_tostr;

fn main() {
    froms();
    intoos();
    trial();
    fromstr();
    tostring()
}
