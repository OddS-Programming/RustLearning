/*
Все типы, для которых требуется std::fmt форматирование traits, нужно имплементировать .
Автоматическая реализация предусмотрена только для таких типов, как в std библиотеке.
 Все остальные должны быть реализованы вручную.

 fmt::Debug trait The makes this very straightforward.
 All types can derive (automatically create) the fmt::Debug implementation.
 This is not true for fmt::Display which must be manually implemented.
 */


// This structure cannot be printed either with `fmt::Display` or то что я показывал ниже, но правильным языком
// with `fmt::Debug`.
#[allow(dead_code)]
struct UnPrintable(i32);

// The `derive` attribute automatically creates the implementation
// required to make this `struct` printable with `fmt::Debug`.
#[allow(dead_code)]
#[derive(Debug)]
struct DebugPrintable(i32);
//======================2==========================
#[allow(dead_code)]// Derive the `fmt::Debug` implementation for `Structure`. `Structure`
// is a structure which contains a single `i32`. Короче структура автоматом имплементируется при помощи derive
#[derive(Debug)]// Так же можно выводить если засунуть структуру в структуру, бутерброд,структура содержит ток одно число
struct Structure(i32);
#[allow(dead_code)]
#[derive(Debug)]
struct Deep(Structure);// Вот он наш бутерброд, гыгы глубина шарите, осьминоги
//=====================3=======================================================
#[allow(dead_code)]
#[derive(Debug)] // Вот уже нормальная структура по прежнему без имплементации но хранит в себе приколы
struct Person<'a> {
    name: &'a str,
    age: u8
}
pub fn debuging(){
    //=========2=============
    // Printing with `{:?}` is similar to with `{}`.
    println!("{:?} months in a year.", 12);
    println!("{1:?} {0:?} is the {actor:?} name.",
             "Slater",
             "Christian",
             actor="actor's");

    // `Structure` is printable!
    println!("Now {:?} will print!", Structure(3));

    // The problem with `derive` is there is no control over how // У derive проблема, нету контроля над тем как оно выводится
    // the results look. What if I want this to just show a `7`?
    println!("Now {:?} will print!", Deep(Structure(7)));// Если чисто 7 захочу вывести то чо делать
    // Как итог fmt::Debug позволяет выводить структуры без явной имплементации но не дает регулировать вывод
    //==========3================
    let name = "Peter";
    let age = 27;
    let peter = Person { name, age };

    // Pretty print
    println!("{:#?}", peter); // Создание экземпляра структуры и вывод его "{:#?}" для красивого вывода без него не выведет
                              //т.к. нет имплементации Display

}

