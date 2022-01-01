fn main() {
  // addition
  let sum = 5 + 10;
  println!("sum: {}", sum);

  // subtraction
  let difference = 95.5 - 4.3;
  println!("difference: {}", difference);

  // multiplication
  let product = 4 * 30;
  println!("product: {}", product);

  // division
  let quotient = 56.7 / 32.2;
  println!("quotient: {}", quotient);
  let floored = 2 / 3; // Results in 0
  println!("floored: {}", floored);

  // remainder
  let remainder = 43 % 5;
  println!("remainder: {}", remainder);

  // tuple type
  let tup = (500, 6.4, 1);

  let (x, y, z) = tup;

  println!("The value of x,y,z is: {}, {}, {}", x, y, z);

  // get tuple value like javascript array
  let tmp2: (i32, f64, u8) = (500, 6.4, 1);

  let tmp2_0 = tmp2.0;
  let tmp2_1 = tmp2.1;
  let tmp2_2 = tmp2.2;

  println!("tmp2: {}, {}, {}", tmp2_0, tmp2_1, tmp2_2);

  // sample array
  let arr = [1, 2, 3, 4, 5];
  println!("arr[0] {}", arr[0]);
  println!("arr[1] {}", arr[1]);

  // Arrayの型指定
  let arr2: [i32; 3];
  // [1, 1, 1]を代入
  arr2 = [1; 3];
  println!("arr2[0] {}", arr2[0]);
}
