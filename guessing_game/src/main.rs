use rand::Rng;
use std::io;

fn main() {
    println!(
        "Угадай число!\n\n1.Чтобы выйти из игры нажмите `q`\n2.Игра закончится, когда вы угадаете число"
    );

    let rand_number = rand::rng().random_range(1..101);
    let max_attempts = 10;
    let mut attempts = 0;

    println!(
        "Загадано число от 1 до 100. У вас {} попыток.",
        max_attempts
    );

    while attempts < max_attempts {
        // println!("Попытка {}/{}: ", attempts + 1, max_attempts);
        println!("Пожалуйста, введите свою догадку: ");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Не получилось прочитать строку");

        if guess.trim() == "q" {
            println!("Выход из игры. Загаданное число было: {}", rand_number);
            return;
        }

        let number: i32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Пожалуйста, введите число или 'q' для выхода!");
                continue;
            }
        };

        attempts += 1;

        if number == rand_number {
            println!(
                "Поздравляем! Вы угадали число {} за {} попыток!",
                rand_number, attempts
            );
            return;
        } else if number > rand_number {
            println!("Меньше!");
        } else {
            println!("Больше!");
        }

        println!("Осталось попыток: {}", max_attempts - attempts);
    }

    println!(
        "Игра окончена, вы не угадали число. Загаданное число было: {}",
        rand_number
    );
}
