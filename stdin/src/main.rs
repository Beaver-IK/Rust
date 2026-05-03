use std::io;

fn main() {
    println!("Пожалуйста введите строку");

    let mut user_input = String::new();

    io::stdin()
        .read_line(&mut user_input)
        .expect("Не удалось прочитать ввод");

    println!("Вы ввели {user_input}");
}
