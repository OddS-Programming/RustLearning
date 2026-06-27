#![allow(dead_code)]

enum Stage {
    Beginner,
    Advanced,
}

enum Role {
    Student,
    Teacher,
}

fn main() {
    // Используем `use` для каждого из вариантов, чтобы они были доступны
    // без указания области видимости. далее по коду будут видны изменения
    use Stage::{Beginner, Advanced}; // тут используем use для конкретных вариантов
    // Автоматически используем `use` для каждого из вариантов в `Work`.
    use Role::*;

    // Equivalent to `Stage::Beginner`.
    let stage = Beginner;
    // Equivalent to `Role::Student`.
    let role = Student;
    // Тоесть нам каждый раз не нужно указывать к чему оно относится, я так понимаю что даже если названия хз че это, тип переменных, объектов
    // будут одинаковыми, мы можем выбирать какой будет так скажем по умолчанию, интересно
    //

    match stage {
        // Обратите внимание, как используются варианты из перечисления `Status`
        // благодаря `use`
        Beginner => println!("Beginners are starting their learning journey!"),
        Advanced => println!("Advanced learners are mastering their subjects..."),
        //вот для примера как использовались варианты в прошлой части темы
        //        WebEvent::PageLoad => println!("page loaded"),
    }

    match role {
        //повторение мать учения
        Student => println!("Students are acquiring knowledge!"),
        Teacher => println!("Teachers are spreading knowledge!"),
    }
}