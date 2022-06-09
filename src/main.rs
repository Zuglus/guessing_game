use std::cmp::Ordering;

use rand::Rng;

fn main() {
    let guessed_number = rand::thread_rng().gen_range(1..=100);

    println!("\nУгадайте число!");
    let mut question = "Ваш вариант?";

    let mut guess_numbers_list: Vec<u32> = Vec::new();

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
                println!("Многовато будет...");
                question = "Поменьше, поменьше.";
            }
            Ordering::Equal => {
                println!("Удивительно!!! Вы угадали!");
                break;
            }
        };

        match guess_numbers_list.len() {
            0 => guess_numbers_list.push(user_guess),
            5 => {
                println!("Ну всё чего ли, игре конец! Загадали {}", guessed_number);
                break;
            }
            _ => {
                print!("Уже были: ");
                println!("{}", guess_numbers_list.iter().fold(String::new(), |acc, &num| acc + &num.to_string() + " "));
                guess_numbers_list.push(user_guess);
            }
        }
    }
}
