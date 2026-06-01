/*
💳 Номерной валидатор 🪤🪤

Для проверки действительности номера кредитной карты может быть использован следующий алгоритм:

    Если сложить все цифры номера банковской карты;
    Добавить к этой сумме каждую вторую цифру, начиная со второй справа;
    Затем к получившейся сумме добавить количество цифр, превышающих четыре из банковского номера,
    также каждой второй, начиная со второй справа, то результат должен нацело делиться на 10.


Если карта валидна, вывести: Карта с номером ???? ???? ???? ???? действительна;
Иначе: Карты с номером ???? ???? ???? ???? не существует.
*/

const LEN_CARD_NUM: u8 = 16;


fn read() -> String {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).expect("Error");
    return input.trim().to_string()
}

fn main() {
    let card_num: u64 = read().parse().expect("Error");
    
    let mut card_digits = [0u8; LEN_CARD_NUM as usize];
    let mut tmp = card_num;

    print!("Карта с номером");

    for i in 0..LEN_CARD_NUM {
        let digit = (tmp % 10) as u8;
        card_digits[(LEN_CARD_NUM - 1 - i) as usize] = digit;
        tmp /= 10;
        if i % 4 == 0 {
            print!(" ");
        } 
        print!("{}", card_digits[i as usize]);
    }

    let mut sum: u8 = 0;
    for i in (0..card_digits.len()).rev() {
        sum += card_digits[i];
        if i % 2 == 0 {
            sum += card_digits[i];
            if card_digits[i] > 4 {
                sum += 1;
            };
        }
    }
    if sum % 10 == 0 {
        print!(" действительна");
    } else {
        print!(" не существует")
    }
}