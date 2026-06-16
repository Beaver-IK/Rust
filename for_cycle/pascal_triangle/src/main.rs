/*
Треугольник Паскаля 🪤🪤

Напишите программу, которая считывает натуральное число n (u16) и выводит первые n строк треугольника Паскаля.
*/
/*

*/

fn read() -> String {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).expect("Ошибка чтения");
    return input.trim().to_string()
}

fn main() {
    let num_lines: u16 = read().parse().expect("Введите натуральное число");

    for i in 0..num_lines {
        let spases = num_lines - i - 1;
        for _ in 0..spases {
            print!(" ");
        }
        let mut val: u16 = 1;
        for j in 0..=i {
            if j != i {
                print!("{val} ");
            } else {
                print!("{val}");
            }
            val = val * (i - j) / (j + 1);
        }
        println!();
    }
}