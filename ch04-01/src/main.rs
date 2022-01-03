fn main() {
  sample_string();
  string_both_equal_sample();
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
  let s2 = s1;

  println!("{}, world!", s1);
}
