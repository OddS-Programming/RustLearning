// Вот тут вам затенение переменных
pub fn shadowing() {
    let shadowed_binding = 1;
    {
        println!("До затенения {}", &shadowed_binding);
        // вот конкретно в этом блоке создаем такую же переменную, ака затеняем прошлую
        let shadowed_binding = "abc";
        println!("во внутреннем блоке она имеет другое значение, {}", shadowed_binding);

    }
    println!("во внешнем блоке: {}", shadowed_binding); // проверяем что снаружи ничего не изменилось

    // Эта привязка *затеняет* предыдущую
    let shadowed_binding = 2;
    println!("затенённая во внешнем блоке: {}", shadowed_binding); 
}