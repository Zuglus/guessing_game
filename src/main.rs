use std::cmp::Ordering;

use rand::Rng;

fn main() {
    let guessed_number = rand::thread_rng().gen_range(1..=100);

    println!("Угадайте число!");
    let mut question = "Ваш вариант?";

    loop {
        println!("{}", question);

        let mut user_guess = String::new();

        std::io::stdin()
            .read_line(&mut user_guess)
            .expect("Ошибка при чтении ввода");

        let user_guess: u32 = match user_guess.trim().parse() {
            Ok(number) => number,
            Err(_) => {
                println!("\nОй! Ой! Число состоит только из циферок)))");
                question = "Вводим число, число!";
                continue;
            }
        };

        println!("\nВы ввели: {}", user_guess);

        match user_guess.cmp(&guessed_number) {
            Ordering::Less => {
                println!("Маловато будет...");
                question = "Что-то побольше?";
            }
            Ordering::Greater => {
                println!("Большевато будет...");
                question = "Поменьше, поменьше.";
            }
            Ordering::Equal => {
                println!("Удивительно!!! Вы угадали!");
                break;
            }
        }
    }
}
