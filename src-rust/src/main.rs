/*fn main() {


    let mut lessThan = false;
    let mut greaterThan = false;
    let mut equalTo = false;
    println!("Please enter your guess");
    let mut guess = String::new();

    let mut random_num1 = rand::thread_rng();
    let random_num = rand::Rng::gen_range(&mut random_num1, 1..=100);

    std::io::stdin()
        .read_line(&mut guess)
        .expect("Could not read input");

    println!("You guessed, {}", guess);
    match guess.cmp(&random_num) {
        std::cmp::Ordering::Less => {
            println!("Too small")
            lessThan = true
            greaterThan = false
            equalTo = false
        }
    }
}
// to_string()

*/

fn main() {
    println!("Hellow rust from cargo ");
}
