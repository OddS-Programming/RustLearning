use std::fmt::format;
use crate::debug::debuging;
use crate::display::display;
use crate::forma::forms;
use crate::formatted_print::formprint;
use crate::testcase::testcase;

// Модули , функции этих модулей по стандарту приватные, но решается pub fn
// а так же могут быть вложенными через nested, чуток забегаю вперед
// если  писать названиемодуля::названиефункции , то можно не париться над одинаковыми названиями функций
mod formatted_print;
mod debug;
pub mod display;
pub mod testcase;
pub mod forma;

fn main() {
    formprint();
    debuging();
    display();
    testcase();
    forms();
}