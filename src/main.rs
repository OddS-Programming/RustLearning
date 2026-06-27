use crate::constants::constants;
use crate::enums::enums;
use crate::structs::structs;
use crate::test_case::test_case;

pub mod structs;
pub mod enums;
pub mod uses;
pub mod c_like;
pub mod test_case;
pub mod constants;

//В языке программирования Rust пользовательские типы данных в основном создаются при помощи двух ключевых слов:
//
// struct: определение структуры
// enum: определение перечисления
// Константы так же могут быть созданы с помощью ключевых слов const и static.
fn main() {
    structs();
    enums();
    test_case();
    constants();
}
