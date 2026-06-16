/*
Умный поезд 🪤🪤

В рамках оптимизации транспортных услуг было принято решение маркировать пассажиров одним числом,
который включал бы его номер и станцию.

Напишите программу, которая считывает количество пассажиров (u16) и столько же натуральных
чисел (u32) представляющие пассажиров.

Значение пассажира содержит в себе его номер, первые 16 бит с конца, а оставшиеся 
16 бит образуют номер станции, на которой выходит пассажир. Программа должна вывести уведомление пассажирам
о просьбе покинуть вагон, как показано в примере.

Так, для четырех пассажиров 196609, 196610, 262147, 262148 выводом будет:

Поезд прибыл на Станцию № 1!
Поезд прибыл на Станцию № 2!
Поезд прибыл на Станцию № 3!
Просим на выход пассажиров с номером(ами):
1, 2
Поезд прибыл на Станцию № 4!
Просим на выход пассажиров с номером(ами):
3, 4
*/


fn read() -> String {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).expect("Ошибка чтения");
    return input.trim().to_string()
}

fn main() {
    let n: u16 = read().parse().expect("Укажите количество пассажиров");

    let mut data: Vec<(u16, u16)> = Vec::with_capacity(n as usize);
    let mut num_stations: u16 = 0;

    for _ in 0..n {
        let value: u32 = read().parse().expect("Введите номер пассажира");

        let station = (value >> 16) as u16;
        let passenger = (value & 0xFFFF) as u16;

        data.push((station, passenger));

        if station > num_stations {
            num_stations = station;
        }
    }
    
    data.sort_by_key(|x| x.0);

    let mut id_pas: u16 = 0;
    for station in 1..=num_stations {
        println!("Поезд прибыл на Станцию № {station}!");
        let mut first: bool = true;
        while id_pas < data.len() as u16 && data[id_pas as usize].0 == station {
            if first {
                println!("Просим на выход пассажиров с номером(ами):");
                first = false;
            } else {
                print!(", ");
            }
            print!("{}", data[id_pas as usize].1);
            id_pas += 1;
        }
        if !first {
            println!();
        }
    }
}