/*
Напишите программу, которая считывает количество часов (u8) отработанных за неделю, а затем выводит (до 2 знаков) заработную плату до вычетов и после, а также сумму налогов, учитывая:

    базовый почасовой тариф: 1500 руб/час.
    переработку более 40 часов в неделю: базовый почасовой тариф * 1.5.
    налоговая ставка: 13%.

Так, если было введено 40 часов, то вывод будет следующим:

Заработная плата до вычетов: 60000.00 руб
Сумма налогов: 7800.00 руб
Заработная плата после вычетов: 52200.00 руб
 */


 use std::io::stdin;


const BASE_RATE: f64 = 1500.00; // 1500 рублей в час (Базовая ставка)
const BASE_RATE_PER_WEEK: f64 = 60000.00; // Стандартная ставка за неделю
const BASE_RATE_OWERTIME: f64 = 2250.00; // 2250 рублей в час (Ставка для переработок)
const STANDART_HOURS_PER_WEEK: f64 = 40.00; // Норма рабочего времени в неделю
const PERSONAL_INCOME_TAX: f64 = 0.13; // НДФЛ 13%

fn read() -> u8 {
    let mut input = String::new();
    stdin().read_line(&mut input).expect("Error 1");
    return input.trim().parse::<u8>().expect("Error 2");
}

fn main() {
    let hours_worked: u8 = read();
    let salary: f64;
    
    if hours_worked as f64 > STANDART_HOURS_PER_WEEK {
        salary = BASE_RATE_PER_WEEK + (hours_worked as f64 - STANDART_HOURS_PER_WEEK) * BASE_RATE_OWERTIME;
    } else {
        salary = hours_worked as f64 * BASE_RATE;
    }
    let tax: f64 = salary * PERSONAL_INCOME_TAX;
    let after_tax: f64 = salary - tax;
    
    println!("Заработная плата до вычетов: {:.2} руб", salary);
    println!("Сумма налогов: {:.2} руб", tax);
    println!("Заработная плата после вычетов: {:.2} руб", after_tax);
}