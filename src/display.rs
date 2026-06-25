use std::fmt;
use std::fmt::write;

//Импорт fmt, std это библиотека стандартная
#[allow(dead_code)]
struct Structure(i32); // Создали структуру которой нужно fnt::Display имплементация, содержит число
impl fmt::Display for Structure { //Ручная имплементация через Display ура ура
    // тоже трейт но для дисплея без которого он не имплементируется
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        //Записывает только первый элемент в указанный output
        // поток 'f' возвращает fmt::Result  который показывает операция успешна или нет,
        write!(f, "{}", self.0)// write! имеет похожий синтаксис что и println!
    }
    //fmt::Display чище выводит чем дебуг , но для библиотеки std возникает проблема, чо делать если тип неоднозначный
    // неоднозначный типо Vec<path>: /:/etc:/home/username:/bin (split on :) этот для путя
                       // Vec<number>: 1,2,3 (split on ,) этот для номеров
    /*
    No, because there is no ideal style for all types and the std library doesn’t presume to dictate one.
    fmt::Display is not implemented for Vec<T> or for any other generic containers.
     fmt::Debug must then be used for these generic cases.
     */
    /*
    Однако это не проблема, потому что для любого нового контейнерного типа,
    который не является универсальным, fmt::Display можно реализовать.
     */

}
#[derive(Debug)]
struct MinMax(i64, i64);// структура которую сейчас имплементируем
impl fmt::Display for MinMax {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({}, {})", self.0, self.1) // Она принимает два числа и выводит потом принимаем результа
    }
}
//попробуем структуру у которой поля с названиями
#[derive(Debug)]
struct Point2D {
    x: f64,
    y: f64, //f64 это float , плавающая точка
}
impl fmt::Display for Point2D {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "x: {}, y: {}", self.x, self.y) // выводить может ток x и y, ставить ; ток если есть последующее действие
    }
} // как кайфово оно само себя пишет я не могу
impl fmt::Binary for Point2D { // такого задания не было но чисто из интереса реализовал
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "x: {:b}, y: {:b}", self.x.to_bits(), self.y.to_bits())
    }
}
#[derive(Debug)]
struct Complex {
    real: f64,
    imag: f64,
}
impl fmt::Display for Complex {//надо пробел после минусы добавить и после пробела
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if self.imag >= 0.0 {
            write!(f, "{} + {}i", self.real, self.imag)
        } else {
            write!(f, "{} - {}i", self.real, self.imag.abs())// .abs() означает что нам нужен модуль числа.
            // тут нам нужен модуль для того что бы мы могли вывести минус с пробелом и они не двоились
        }
    }
}
pub fn display() {
    let minmax = MinMax(0, 14);
    println!("Compare structures:");
    println!("Display: {}", minmax);
    println!("Debug: {:?}", minmax);//{:?} Означает что дебугом выводим
    let big_range = MinMax(-300, 300);
    let small_range = MinMax(-3, 3);
    println!("The big range is {big} and the small is {small}",
    small = small_range,
    big = big_range);
    let point = Point2D { x: 3.3, y: 7.2 };
    println!("Compare points:");
    println!("Display: {}", point);
    println!("Debug: {:?}", point);//{:?} Означает что дебугом выводим
    /* Тут дисплей для вывода проверяет положительное второе число или отрицательное
        что бы после вывода был либо плюс либо минус*/
    let complex = Complex { real: 3.3, imag: 7.2 };
    let ocomplex = Complex { real: 4.7, imag: -2.3};
    println!("Compare Complexs:");
    println!("Display: {}", complex);
    println!("Debug: {:?}", complex);
    println!("Display: {}", ocomplex);
    println!("Debug: {:?}", ocomplex);
    println!("Debug: {:b}", point);// реализовано ради хиханек и хаханек и вывод не правильный



}



