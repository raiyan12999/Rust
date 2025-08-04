fn main() {
    let x = 34;
    let y = 56;

    let z = x + y;

    println!("{}", z);

    let mut i = 0;

    while i < 6 {
        println!("I am trying to learn Rust");
        i = i + 1;
    }

    // code to add numbers from 1 to 100
    let mut sum = 0;

    for i in 1..100 {
        sum += i;
        println!("Sum: {}", sum);
    }
}
