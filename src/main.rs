fn main(){
    let mut x = 3;

    println!("The value of x was initially {}", x);

    x = 9;

    println!("The value of x is now {}", x);

    // This became possible because of mut keyword. Otherwise Rust variables are immutable

    // Type casting
    // We use 'as' keyword for changing one variable of one type to another type

    let decimal = 43.35;
    println!("The decimal value of the variable is {}", decimal);

    let integer = decimal as u64;
    println!("The integer value of the variable is {}", integer);

    let another_integer = decimal as i128;
    println!("The new integer value of the variable is {}", another_integer);


}