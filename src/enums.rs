/*Ключевое слово enum позволяет создавать тип данных,
который представляет собой один из нескольких возможных вариантов.
Любой вариант, действительный как struct, также действителен как enum.
 */
// Создаём `enum` для классификации web-событий. Обратите внимание,
// как имена и информация о типе определяют вариант:
// `PageLoad != PageUnload` и `KeyPress(char) != Paste(String)`.
// Все они разные и независимые.

#![allow(dead_code)]
// struct
// struct используют, когда нужно сгруппировать связанные данные в единый объект.
// Все поля структуры всегда присутствуют в памяти — вы не можете «выбрать» только часть из них.

// enum (очень интересная тема, стоит запомнить)
// (перечисление) нужен, когда значение может быть одним из нескольких заранее определённых вариантов.
// В памяти хранится только один вариант из всех возможных, что экономит ресурсы
enum WebEvent {
    // `enum` может быть как `unit-подобным`, ну считай пустым
    PageLoad,
    PageUnload,
    // так и кортежной структурой
    KeyPress(char),
    Paste(String),
    // или С-подобной структурой.
    Click { x: i64, y: i64 },
}
//Функция которая принимает enum как аргумент и ничего не возвращает
fn inspect(event: WebEvent) { // Можно думаю брать аналогию с Display, но тут мы не просто вывод ему записываем, а конкретно его поведение
    match event { // Очень крутой механизм управления потоком что то типо SwitchCase но для шаблонов , если шаблоны совпали, то делает нужный код,
        // интересно нужно углубиться
        //вызываем PageLoad и PageUnload, т.к. они у нас unit подобные, тоесть пустые, можем делать с ними что душе угодно
        WebEvent::PageLoad => println!("page loaded"),
        WebEvent::PageUnload => println!("page unloaded"),
        // Деструктуризируем ака извлекаем c из enum
        WebEvent::KeyPress(c) => println!("pressed '{}'", c),
        WebEvent::Paste(s) => println!("pasted \"{}\"", s),
        //теперь деструктурируем клик на x и y
        WebEvent::Click { x, y } => println!("clicked at x={}, y={}", x, y),
    }
}
// Поскольку это все по идее кастомные типы данных, которые невероятно красиво и понятно интегрированы в раст
// И называть их можно как угодно, так что если название слишком длинное, непонятное и т.д.
// Можно ему название что то типо переопределить, для удобства
enum VeryVerboseEnumOfThingsToDoWithNumbers { // Вот наш слишком длинно называнный enum
    Add,
    Subtract,
}
type Operations = VeryVerboseEnumOfThingsToDoWithNumbers; // даем ему псевдоним другой, переименовываем
// Если брать имплементацию, то можно использовать Self:: то бы не загрязнять код
impl VeryVerboseEnumOfThingsToDoWithNumbers {
    fn run(&self, x: i32, y: i32) -> i32 {
        match self {
            Self::Add => x + y,
            Self::Subtract => x - y,
        }
    }
}

pub fn enums() {
    // а теперь демонстрация работы
    let pressed = WebEvent::KeyPress('x'); // Имитируем нажатие на кнопку
    let pasted  = WebEvent::Paste("my text".to_owned()); // `to_owned()` создаст `String` из строкового среза.
    let click   = WebEvent::Click { x: 20, y: 80 }; // Имитируем клик
    let load    = WebEvent::PageLoad;// Имитируем загрузку страницы
    let unload  = WebEvent::PageUnload;// Имитируем отгрузку угадайте чего
    inspect(pressed);
    inspect(pasted);
    inspect(click);
    inspect(load);
    inspect(unload);
    let x = Operations::Add;
    x.run(1, 2);  // чисто что бы не било предупреждение
}