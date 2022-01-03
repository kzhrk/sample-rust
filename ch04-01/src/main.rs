fn main() {
  sample_string();
  string_both_equal_sample();

  let s1 = gives_ownership();

  let s2 = String::from("hello");

  let s3 = takes_and_gives_back(s2);

  println!("s1: {}", s1);
  // s2はtakes_and_gives_backで参照が切れているのでprintするとコンパイルエラーになる
  // println!("s2: {}", s2);
  println!("s3: {}", s3);

  let (s4, len) = calculate_length(s3);
  println!("The length of '{}' is {}.", s4, len);
}

fn sample_string() {
  let mut s = String::from("hello");

  s.push_str(", world!"); // push_str() appends a literal to a String

  println!("{}", s); // This will print `hello, world!`
}

// compile error
//
// error message:
// let s1 = String::from("hello");
//     -- move occurs because `s1` has type `String`, which does not implement the `Copy` trait
// let s2 = s1;
//          -- value moved here
//
// println!("{}, world!", s1);
//                        ^^ value borrowed here after move
fn string_both_equal_sample() {
  let s1 = String::from("hello");
  let s2 = s1.clone();

  println!("{} world!", s1);
  println!("{} world!", s2);
}

fn gives_ownership() -> String {
  let some_string = String::from("yours");
  some_string
}

fn takes_and_gives_back(a_string: String) -> String {
  a_string
}

fn calculate_length(s: String) -> (String, usize) {
  let length = s.len(); // len() returns the length of a String

  (s, length)
}
