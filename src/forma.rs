use std::fmt::{self,Formatter,Display}; // formatter и display взяли из fmt
//форматирование достигается макросом format!
// format!("{}", foo) -> "3735928559"
// format!("0x{:X}", foo) ->"0xDEADBEEF"
// format!("0o{:o}", foo) -> "0o33653337357"

//Функционал форматирования реализован благодаря типажу,
// и для каждого типа аргумента существует свой.
// Наиболее распространённый типаж для форматирования — Display,
// который работает без аргументов: например {}.

struct City { // структура с разными типами данных
    name: &'static str,
    lat: f32,// Широта
    lon: f32,// Долгота
}
impl Display for City { // имплементируем Display для City
    // `f` — это буфер, данный метод должен записать в него форматированную строку
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        let lat_c = if self.lat >= 0.0 { 'N' } else { 'S' }; // В зависимости от широты и долготы как я понял
        let lon_c = if self.lon >= 0.0 { 'E' } else { 'W' };// выбираем сторону света
        // `write!` похож на `format!`, только он запишет форматированную строку
        // в буфер (первый аргумент функции)
        write!(f, "{}: {:.3}°{} {:.3}°{}",
               self.name, self.lat.abs(), lat_c, self.lon.abs(), lon_c) // Выводим название, модуль широты, сторону света
        //модуль долготы и сторону света
    }
}
#[derive(Debug)]
struct Color { // структура с кодом цвета
    red: u8,
    green: u8,
    blue: u8,
}
impl Display for Color {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {

        write!(f, "RGB({0}, {1}, {2}) 0x{0:02X}{1:02X}{2:02X}", self.red, self.green, self.blue)
    }

}
pub fn forms(){
    for city in [ // каждый город списке отделяем итератором и выводим
        City { name: "Дублин", lat: 53.347778, lon: -6.259722 },
        City { name: "Осло", lat: 59.95, lon: 10.75 },
        City { name: "Ванкувер", lat: 49.25, lon: -123.1 },
    ].iter() {
        println!("{}", *city);
    }
    for color in [// тоже самое что и с городами
        Color { red: 128, green: 255, blue: 90 },
        Color { red: 0, green: 3, blue: 254 },
        Color { red: 0, green: 0, blue: 0 },
    ].iter() {
        println!("{}", color)
    }
}