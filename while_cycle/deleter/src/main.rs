/*
Deleter 🪤🪤

Напишите программу, которая считывает два натуральных (u32) числа n и k соответственно,
а затем выводит число, полученное удалением первых k цифр из числа n. 
Если количество удаляемых цифр нестрого больше, чем количество цифр в n, то выводится сообщение k >= n.
*/


fn read() -> String {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).expect("Error");
    input.trim().to_string()
}


fn main() {
    let n_str: String = read();
    let k: u32 = read().parse().expect("Error 2");

    match n_str.len() <= k as usize {
        true => {
            println!("k >= n");
        }
        false => {
            let n: u32 = n_str.parse().expect("Error 3");
            println!("{}", n % 10_u32.pow(k));
        }
    }
}
