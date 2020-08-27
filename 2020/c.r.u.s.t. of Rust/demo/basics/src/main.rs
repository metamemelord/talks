fn main() {
    let x = "people!";
    println!("Hello, {}", x);

    for it in 0..10 { // Iteration from 0 to 9
        println!("{} ", it);
    }

    let v: Vec<i32> = vec![1, 4, 6]; // Syntactical sugar for Vec<i32>
    println!("{:?}", v);
}