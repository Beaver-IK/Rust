/*Проверка установки i-го бита 🪤🪤

Напишите программу, которая считывает два целых числа x (u8) и n (u8), проверяет на установку n-й бит переменной x и выводит получившееся число в двоичной записи в виде сообщения:

{n}-й бит числа {:b} равен {} */


fn main() {
    let x: u8;
    let n: u8;
    
    let mut input = String::new();
    
    std::io::stdin().read_line(&mut input).expect("Error1");
    x = input.trim().parse().expect("Error2");
    input.clear();
    
    std::io::stdin().read_line(&mut input).expect("Error1");
    n = input.trim().parse().expect("Error2");
    input.clear();
    
    println!("{n}-й бит числа {x:b} равен {}", (x >> n) & 1);
}