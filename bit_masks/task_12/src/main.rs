/*
Очистка всех битов от старшего до i-го бита 🪤🪤

Напишите программу, которая считывает два целых числа x (i8) и n (u8), очищает все биты от старшего до n-го у переменной x и выводит получившееся число в двоичной и десятичной записи в виде сообщений:

x до очистки от 7 до {n}-го бита
в двоичной записи: {:b}
в десятичной записи: {}

x после очистки от 7 до {n}-го бита
в двоичной записи: {:b}
в десятичной записи: {}
*/


fn main() {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).expect("Error");
    let x: i8 = input.trim().parse().expect("Error");
    input.clear();
    
    std::io::stdin().read_line(&mut input).expect("Error");
    let n: u8 = input.trim().parse().expect("Error");

    // Формула: (1 << n) - 1 (оставляет биты 0..(n-1))
    let mask = (1 << n) - 1;
    let result = x & mask as i8;
    
    println!("x до очистки от 7 до {}-го бита", n);
    println!("в двоичной записи: {:08b}", x);
    println!("в десятичной записи: {}", x);
    println!();
    println!("x после очистки от 7 до {}-го бита", n);
    println!("в двоичной записи: {:08b}", result);
    println!("в десятичной записи: {}", result);
}