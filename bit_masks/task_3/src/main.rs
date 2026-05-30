/*Переключение i-го бита 🪤🪤

Напишите программу, которая считывает два целых числа x (i8) и n (u8), переключает n-й бит переменной x и выводит получившееся число в двоичной и десятичной записи в виде сообщений:

x до переключения {n}-го бита
в двоичной записи: {:b}
в десятичной записи: {}

x после переключения {n}-го бита
в двоичной записи: {:b}
в десятичной записи: {} */


fn main() {
    let x: i8;
    let n: u8;
    
    let mut input = String::new();
    
    std::io::stdin().read_line(&mut input).expect("Error1");
    x = input.trim().parse().expect("Error2");
    input.clear();
    
    std::io::stdin().read_line(&mut input).expect("Error1");
    n = input.trim().parse().expect("Error2");
    input.clear();
    
    println!("x до переключения {}-го бита", n);
    println!("в двоичной записи: {:08b}", x);
    println!("в десятичной записи: {}\n", x);
    
    println!("x после переключения {}-го бита", n);
    println!("в двоичной записи: {:08b}", x ^ (1 << n));
    println!("в десятичной записи: {}", x ^ (1 << n));
}