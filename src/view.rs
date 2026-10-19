use std::io::{self,Write};


fn prompt_parse(message: &str) -> i32 {
    loop {
        println!("{message}");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin().read_line(&mut  input).expect("Ошибка чтения");

        match input.trim().parse::<i32>() {
            Ok(value) if value > 0 => return value,
            _ => println!("Ошибка:: введите положительное число")
        }
    }
}