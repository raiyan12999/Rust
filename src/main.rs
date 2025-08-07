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
        
    }

    println!("Sum: {}", sum);

    // Array without data type
    let a = [1, 3, 5, 6, 9];

    // Array with data type and size
    let b: [i32; 6] = [2, 4, 5, 6, 7, 9];

    // Array with default values
    let c = [3; 5];

    println!("{:?}", a);
    println!("{:?}", b);
    println!("{:?}", c);

    
    println!("Try programiz.pro");
    print!("Trying");
    println!(" Trying hard");
    
    let name = "Raiyan";
    let age = 27;
    
    println!("My name is {1}, My age is {0}", name, age);

}
