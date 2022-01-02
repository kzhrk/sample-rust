fn main() {
  let number = 3;

  if number < 5 {
    println!("condition was true");
  } else {
    println!("condition was false");
  }

  // using if in a let statement
  let condition = true;
  let number = if condition { 5 } else { 6 };

  println!("The value of number is: {}", number);

  sample_loop();
  return_values_from_loop();
  sample_while();
}

fn sample_loop() {
  let mut count = 0;
  'counting_up: loop {
    println!("count = {}", count);
    let mut remaining = 10;

    loop {
      println!("remaining = {}", remaining);
      if remaining == 9 {
        break;
      }
      if count == 2 {
        break 'counting_up;
      }
      remaining -= 1;
    }

    count += 1;
  }
  println!("End count = {}", count);
}

fn return_values_from_loop() {
  let mut counter = 0;

  let result = loop {
    counter += 1;

    if counter == 10 {
      break counter * 2;
    }
  };

  println!("The result is {}", result);
}

fn sample_while() {
  let mut number = 3;

  while number != 0 {
    println!("{}!", number);

    number -= 1;
  }

  println!("LIFTOFF!!!");
}
