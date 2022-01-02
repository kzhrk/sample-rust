fn main() {
  another_function(5);
  print_labeled_measurement(5, 'h');
  fn_bodies_contain_logic();
}

fn another_function(x: i32) {
  println!("The value of x is: {}", x);
}
fn print_labeled_measurement(value: i32, unit_label: char) {
  println!("The measurement is: {}{}", value, unit_label);
}

fn fn_bodies_contain_logic() {
  let y = {
    let x = 3;
    x + 1
  };
  println!("The value of y is: {}", y);
}
