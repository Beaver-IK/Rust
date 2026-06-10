/*
 Недостающее число 🪤

Напишите программу, которая считывает натуральные числа (u32), 
пока не будет введено иное, а затем выводит недостающее число введенной
последовательности в виде сообщения: Пропущено число {}.

Гарантируется, что количество чисел в последовательности всегда на 1 меньше и стоп строка не равна пробельным символам!
*/


use std::collections::HashSet;

fn read() -> String {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).expect("Error");
    input.trim().to_string()
}

fn main() {
    let mut numbers: HashSet<u32> = HashSet::new();
    let mut reading: bool = true;
    
    while reading {
        let line = read();
        match line.parse::<u32>() {
            Ok(num) => {
                numbers.insert(num);
            }
            Err(_) => {
                let mut missing = 1;
                while numbers.contains(&missing) {
                    missing += 1;
                }
                println!("Пропущено число {}", missing);
                reading = false;
            }
        }
    }
}