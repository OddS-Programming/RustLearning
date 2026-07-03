// По базе связывание переменных неизменяемо , решаемо с помощью идентификатора mut
// Ну типо mutable шарите
pub fn mutanti(){
    let _immutable_binding = 1;
    let mut mutable_binding = 1;
    println!("до изменения {}", mutable_binding);
    mutable_binding += 1;
    println!("после измене {}", mutable_binding);

    //_immutable_binding +=1; // а вот раскоментируйте эту строку , ругаться будет
    //а еще компилятор будет выводить подробные сообщения об ошибках, связанных с изменяемостью.
    //Ну красота же
}