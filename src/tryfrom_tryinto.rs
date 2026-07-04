//Как и From и Into, TryFrom и TryInto - обобщённые типажи для конвертации между типами.
// Но в отличии от From/Into, типажи TryFrom/TryInto используются для преобразований с ошибками и возвращают Result.

// Простыми словами то же самое но обернутое в try/catch
use std::convert::TryFrom;
use std::convert::TryInto;

#[derive(Debug, PartialEq)] // PartialEq трейт если нам нужно проверить равно ли одно другому
struct EvenNumber(i32);

impl TryFrom<i32> for EvenNumber {
    type Error = ();

    fn try_from(value: i32) -> Result<Self, Self::Error> {
        if value % 2 == 0 {
            Ok(EvenNumber(value))
        } else {
            Err(())
        }
    }
} // Прикольно что мы можем контролировать преобразование в то что нам надо

pub fn trial() {
    // TryFrom

    assert_eq!(EvenNumber::try_from(8), Ok(EvenNumber(8)));
    assert_eq!(EvenNumber::try_from(5), Err(()));

    // TryInto

    let result: Result<EvenNumber, ()> = 8i32.try_into();
    assert_eq!(result, Ok(EvenNumber(8)));
    let result: Result<EvenNumber, ()> = 5i32.try_into();
    assert_eq!(result, Err(()));
}
