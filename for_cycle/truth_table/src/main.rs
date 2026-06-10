/*
Таблица истинности 🪤🪤

Напишите программу, которая считывает натуральное число n (usize) и выводит
таблицу истинности для операции побитового И размерностью n.

Так, для n = 1 выводом будет:
0 | 0
1 | 1

 Еще пример, для n = 2 выводом будет:
00 | 0
01 | 0
10 | 0
11 | 1
*/

fn read() -> String {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).expect("Error");
    input.trim().to_string()
}

fn main() {
    let n: i32 = read().parse().expect("Error");

    let max = 1 << n;

    for i in 0..max {
        print!("{i:0width$b} | ", width = n as usize);
        if i == max - 1 {
            println!("1");
        } else {
            println!("0");
        }
    }
}