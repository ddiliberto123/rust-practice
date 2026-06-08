use std::io;

fn main() {
    let number = 6;

    if number % 4 == 0 {
        println!("number is divisible by 4");
    } else if number % 3 == 0 {
        println!("number is divisible by 3");
    } else if number % 2 == 0 {
        println!("number is divisible by 2");
    } else {
        println!("number is not divisible by 4, 3, or 2");
    }

    let condition = true;
    let number = if condition { 5 } else { 6 };

    println!("The value of number is: {number}");
    
//    convert();
   println!("Fibonacci number is {}", fibonacci());
}

fn convert() -> () {
    let mut unit = String::new();
    let mut temp = String::new();

    println!("Fahrenheight or Celsius (f/c): ");

    io::stdin()
        .read_line(&mut unit)
        .expect("Failed to read line");

    let is_fahrenheight = unit.to_lowercase().chars().next() == Some('f');

    println!("Temprature: ");

    io::stdin()
        .read_line(&mut temp)
        .expect("Failed to read line");

    let temp: i32 = temp.trim().parse().expect("Could not parse");

    if is_fahrenheight {
        println!("Value in Celsius is {}", ((temp - 32) as f32 * (5.0/9.0)).floor())
    } else {
        println!("Value in Fahrenheight {}", (temp as f32 - (5.0/9.0)).floor() + 32.0)
    }
}

fn fibonacci() -> u64 {
    let mut num = String::new();

    println!("Fibonacci number: ");

    io::stdin()
        .read_line(&mut num)
        .expect("Failed to read line");
    
    let num: u32 = num.trim().parse().expect("Could not parse");

    let mut left: u64 = 0;
    let mut right: u64 = 1;
    for _ in 0..num {
        let temp = left + right;
        left = right;
        right = temp;
        print!("{left} ");
    }
    println!();
    left
}