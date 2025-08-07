fn main(){
    let mut x = 3;

    println!("The value of x was initially {}", x);

    x = 9;

    println!("The value of x is now {}", x);

    // This became possible because of mut keyword. Otherwise Rust variables are immutable
}