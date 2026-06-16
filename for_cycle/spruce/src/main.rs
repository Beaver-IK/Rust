/*
🌲 Ёлочка 🌲

Напишите программу, которая считывает натуральное число n (u8) и выводит треугольник из * с высотой равным n.
*/



fn read() -> String {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).expect("Error");
    return input.trim().to_string()
}

fn main() {
    let n: u8 = read().parse().expect("Введите число");

    for i in 0..n {
        for _ in 0..n - i - 1 {
            print!(" ");
        }
        for _ in 0..i * 2 + 1{
            print!("*");
        }
        println!();
    }
}