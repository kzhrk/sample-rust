use std::io;

fn main() {
  println!("Guess the number!");

  println!("Please input your guess.");

  let mut guess = String::new();

  io::stdin()
    .read_line(&mut guess)
    .expect("Failed to read line");

  println!("You guessed: {}", guess);

  let mut guess2 = String::new();
  io::stdin()
    .read_line(&mut guess2)
    .expect("Failed to read line");

  println!("You guessed: {}, {}", guess, guess2);
}
