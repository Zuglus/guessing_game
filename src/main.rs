use std::cmp::Ordering;

use rand::Rng;

fn main() {
    let guessed_number = rand::thread_rng().gen_range(1..=100);
    println!("Загаданное число: {}", guessed_number);

    println!("Угадайте число!");
    println!("Ваш вариант?");

    let mut user_guess = String::new();

    std::io::stdin()
        .read_line(&mut user_guess)
        .expect("Ошибка при чтении ввода");

    let user_guess: u32 = user_guess.trim().parse().expect("Необходимо ввести число!!!");

    println!("Вы ввели: {}", user_guess);

    match user_guess.cmp(&guessed_number) {
        Ordering::Less => println!("Маловато будет..."),
        Ordering::Greater => println!("Большевато будет..."),
        Ordering::Equal => println!("Удивительно!!! Вы угадали!"),
    }
}
