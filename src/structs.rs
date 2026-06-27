/*
Существует три типа структур, которые можно создать с помощью ключевого слова struct:

Кортежная структура, которая на самом деле является именованным кортежем.
Классическая C-структура
Единичная структура, которая не имеет полей, но может быть полезна для обобщённых типов.
 */
#![allow(dead_code)]
#[derive(Debug)]// обычная структура
struct Person {
    name: String,
    age: u8,
}

struct Unit; // Пустая unit структура, нужна для обобщщенных типов
struct Pair(i32, f32);//Кортежная структура
#[derive(Debug)]
struct Point { // структура с двумя полями
    x: f32,
    y: f32,
}

// Структуры могут быть использованы в качестве полей другой структуры
#[allow(dead_code)]
#[derive(Debug)]
struct Rectangle {
    // Прямоугольник может быть определён по расположению в пространстве
    // его верхнего левого и нижнего правого углов
    top_left: Point,
    bottom_right: Point,
}
fn rect_area(rect: Rectangle) -> f32 { // нужно создать функцию которая вычисляет плозадь прямоугольника в ней можно попробовать
    //вложенную деструктуризацию, ану
    // под двум точкам, левой верхней и правой нижней формула площади такая s = (x2 - x1) * (y1 - y2);
    // тоесть нам надо деструктурировать Rectangle на корды
    let Rectangle { top_left, bottom_right } = rect;
    let width = bottom_right.x - top_left.x; // По формуле узнаем ширину
    let height = bottom_right.y - top_left.y;//  длинну
    (width * height) as f32 // И возвращаем площадь конвертируя её в f32
}
fn square_two(point: Point, width: f32, height:f32) -> Rectangle { // тут задание создать функцию square которая принимает Point и f32,
    //  возвращает rectangle где левый верхний угол это Point? а ширина и высота f32
    let top_left = point; // левая верхняя точка уже известна,
//    let Point{x, y} = top_left; думал через деструктуризацию сделать но нашел способ проще
    let bottom_right = Point{x:width, y:height};// вот так, тупо создаем поинт с полями
    Rectangle {top_left, bottom_right}// ну и возвращаем прямоугольник
    //конкретно задание я не понял то ли один f32 надо то ли два, это вариант с двумя, сейчас сделаю с одним
    //Я дурак, не вник в название функции, нам квадрат нужен, так что высота и ширина будет одинаковой,
    //Сделаем вид что этот позор никто не видел
}
fn square_one(point: Point, sec: f32) -> Rectangle { // тут задание создать функцию square которая принимает Point и f32,
    //  возвращает rectangle где левый верхний угол это Point? а ширина и высота f32
    let top_left = point; // левая верхняя точка уже известна,
    //    let Point{x, y} = top_left; думал через деструктуризацию сделать но нашел способ проще
    let bottom_right = Point{x:sec, y:sec};// вот так, тупо создаем поинт с полями
    Rectangle {top_left, bottom_right}// ну и возвращаем квадрат получается
}
pub fn structs(){
    let name = String::from("Peter"); // сокращенная инициализация полей
    let age = 27;
    let peter = Person { name, age };

    println!("{:?}", peter); //Вывод с помощью debug, она же отладочная информация о структуре
    // Инициализируем Point
    let point: Point = Point { x: 10.3, y: 10.4 };
    //Получаем доступ к полям структуры
    println!("координаты точки: ({}, {})", point.x, point.y);
    // Создадим новую точку, используя синтаксис обновления структуры и нашу существующую точку
    let bottom_right = Point { x: 5.2, ..point }; //мы изменили x на 5.2 ,
    // а ..point значит что мы сохранили предыдущее значение
    println!("вторая точка: ({}, {})", bottom_right.x, bottom_right.y); // Вот вывод для наглядности bottom_right.x = 5.2
    //а bottom_right.y такой же как point.y
    // используя let мы деструктурируем, структуру, это значит что мы разобрали сложную структуру на отдельные переменные
    let Point { x: top_edge, y: left_edge } = point;

    let _rectangle = Rectangle {
        // создание структуры также является выражением
        top_left: Point { x: left_edge, y: top_edge },
        bottom_right: bottom_right,
    }; //В Rust выражение — это конструкция, которая вычисляется и возвращает значение.
    // Ключевое отличие от оператора в том, что выражение возвращает результат,
    // а оператор (statement) просто выполняет действие и не возвращает значение.

    // Instantiate a unit struct
    let _unit = Unit;// создаем юнит структуру ака пустую

    // Instantiate a tuple struct
    let pair = Pair(1, 0.1);// кортежная структура ака кортеж с именем

    // Выводим значения из картежной структуры
    println!("pair contains {:?} and {:?}", pair.0, pair.1);


    let Pair(integer, decimal) = pair; //теперь мы ее деструктируем, разбивая на integer и decimal

    println!("pair contains {:?} and {:?}", integer, decimal);// выводим новые переменные по отдельности

    let point1: Point = Point { x: 10.3, y: 10.4 };
    let point2: Point = Point { x: 12.2, y: 9.0 };
    let point3: Point = Point { x: 12.2, y: 9.0 };
    // Проверяем задание на готовность
    let recto: Rectangle = Rectangle{top_left:point1, bottom_right:point2};
    println!("Площадь прямоугольника: {}",rect_area(recto));
    //тестим создание квадрата
    println!("Наш квардрат: {:?}" ,square_one(point3,13.0));


}