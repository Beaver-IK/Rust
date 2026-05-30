/*Установка i-го бита 🪤🪤

Напишите программу, которая считывает два целых числа x (i8) и n (u8), устанавливает n-й бит переменной x и выводит получившееся число в двоичной и десятичной записи в виде сообщений:

x до установки {n}-го бита
в двоичной записи: {:b}
в десятичной записи: {}

x после установки {n}-го бита
в двоичной записи: {:b}
в десятичной записи: {}*/


fn main() {
    let mut x: i8;
    let n: u8;
    
    let mut input = String::new();
    
    std::io::stdin().read_line(&mut input).expect("Error1");
    x = input.trim().parse().expect("Error2");
    input.clear();
    
    std::io::stdin().read_line(&mut input).expect("Error1");
    n = input.trim().parse().expect("Error2");
    input.clear();
    
    println!("x до установки {}-го бита", n);
    println!("в двоичной записи: {:08b}", x);
    println!("в десятичной записи: {}\n", x);
    
    x |= 1 << n;
    
    println!("x после установки {}-го бита", n);
    println!("в двоичной записи: {:08b}", x);
    println!("в десятичной записи: {}", x);
}