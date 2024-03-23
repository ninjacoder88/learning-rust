fn main(){
    //variable in rust are immutable by default
    //they can be made mutable using the mut keyword
    let mut x = 5;
    //println is a function, println! is a macro (to be described later)
    println!("THe value of x is {x}");
    x = 6;
    println!("THe value of x is {x}");

    //const is data that be be determined at compile time and must have a type
    const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;
    println!("{THREE_HOURS_IN_SECONDS}");

    //let, similar to const, is determined at compile time, but a type is not explicit
    let y = 5;

    //variables can be shadowed (redefined with new data)
    let y = y + 1;

    //this new value with be effective with the scope of that shadow
    //a new scope can be introduced using { }
    {
        let y = y * 2;
        println!("The value of y in the inner scope is {y}");
    }

    println!("The value of y is {y}");

    //shadowing lets variables take on new types
    let spaces = "    ";
    let spaces = spaces.len();
    println!("{spaces}");
}