fn show(i: &i32) {
  println!("{}", i);
}

fn main() {
  let x = 10;
  show(&x);
  println!("{}", x)
}
