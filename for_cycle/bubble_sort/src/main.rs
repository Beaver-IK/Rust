/*
Сортировка по возрастанию

Напишите программу, которая считывает десять целых чисел,
а затем выводит с помощью {:?} получившуюся последовательность по возрастанию.
*/


const NUM_DIGITS: usize = 10;


fn read() -> String {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).expect("Error");
    return input.trim().to_string()
}

fn main() {
    let mut digits = [0i32;10];

    for i in 0..NUM_DIGITS {
        digits[i] = read().parse().expect("Введите число");
    }

    for _ in 0..NUM_DIGITS {
        for j in 0..NUM_DIGITS - 1 {
            if digits[j] > digits[j + 1] {
                digits[j] = digits[j] ^ digits[j + 1];
                digits[j + 1] = digits[j] ^ digits[j + 1];
                digits[j] = digits[j] ^ digits[j + 1];
            }
        }
    }

    println!("{:?}", digits);
}