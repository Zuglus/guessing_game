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

    println!("Вы ввели: {}", user_guess);
}
