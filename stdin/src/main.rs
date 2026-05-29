fn main() {
    let mut x = String::new();
    let mut y = String::new();
    
    std::io::stdin()
        .read_line(&mut x)
        .expect("Err");
    std::io::stdin()
        .read_line(&mut y)
        .expect("Err");
    
    println!("{}", x.trim().parse::<i16>().unwrap() + y.trim().parse::<i16>().unwrap() -
        x.trim().parse::<i16>().unwrap());
    println!("{}", y.trim().parse::<i16>().unwrap() + x.trim().parse::<i16>().unwrap() -
        y.trim().parse::<i16>().unwrap());
}