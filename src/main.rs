use std::io;
use std::str::FromStr;

fn main() {
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Не удалось прочитать строку");

    let numbers: Vec<i32> = input
        .trim()
        .split(", ")
        .map(|s| i32::from_str(s).expect("Не удалось преобразовать строку в число"))
        .collect();
    
    let mut sum = 0;

    for &number in &numbers {
        sum += number;
    }

    let mean = sum as f32 / numbers.len() as f32;

    println!("Сумма равна {sum}, арифметическое среднее равно {mean}")
}