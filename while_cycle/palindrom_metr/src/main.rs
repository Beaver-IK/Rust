/*
📏 Палиндромер 🪤🪤

Напишите программу, которая считывает натуральное число (u64) и выводит,
используя его цифры наибольший возможный числовой палиндром.
Если палидром невозможно образовать, вывести: Число {} не образует палиндром.
*/


use std::io::stdin;


fn read() -> String {
    let mut input = String:: new();
    stdin().read_line(&mut input).expect("Ошибка ввода");
    input.trim().to_string()
}

fn main() {
    let n: u64 = read().parse().expect("Неодбходимо ввести число");
    let mut digits: Vec<u8> = Vec::new();
    let mut temp: u64 = n;

    while temp > 0 {
        digits.push((temp % 10) as u8);
        temp /= 10;
    }

    let n_len: usize = digits.len();

    if n_len <= 1 {
        println!("Число {n} не образует палиндром");
        return;
    }

    let is_even: bool = n_len % 2 == 0; 
    let max_singles: u8 = if is_even{0} else {1};
    let mut middle: u8 = 0;

    digits.sort();

    let mut pairs_stack: Vec<u8> = Vec::new();
    let mut single_count: u8 = 0;

    let mut i: usize = 0;

    while i < n_len {
        if i + 1 < n_len && digits[i] == digits[i + 1] {
            pairs_stack.push(digits[i]);
            i += 2;
        } else {
            single_count += 1;
            if single_count > max_singles {
                println!("Число {n} не образует палиндром");
                return;
            }
            middle = digits[i];
            i += 1;
        }
    }

    for digit in pairs_stack.iter().rev() {
        print!("{digit}");
    }
    if !is_even {
        print!("{middle}");
    }
    for digit in pairs_stack.iter() {
        print!("{digit}");
    }
    println!();
}