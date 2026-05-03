use std::io;

fn main() {
    println!("Пожалуйста введите строку");

    let mut user_input = String::new();

    io::stdin()
        .read_line(&mut user_input)
        .expect("Не удалось прочитать ввод");

    let in_num: i32 = user_input
        .trim() // метод trim удаляет из строки все пробельные символы, включая \n
        .parse() // метод parse преобразует строку в другой тип
        .expect("Пожалуйста введите число");

    println!("{in_num} + 1 = {}", in_num + 1);
}
