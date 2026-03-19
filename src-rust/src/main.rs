fn main() {
    println!("Please enter your guess");
    let mut guess = String::new();

    let mut random_num1 = rand::thread_rng();
    let random_num = rand::Rng::gen_range(&mut random_num1, 1..=100);

    std::io::stdin()
        .read_line(&mut guess)
        .expect("Could not read input");

    println!("You guessed, {}", guess);
}
// to_string()
