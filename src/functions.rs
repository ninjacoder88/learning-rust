fn main() {
    println!("Hello, world!");

    another_function(5);

    expressions();

    let x = five();
    println!("The value of x is {x}");

    let y = plus_one(5);
    println!("The value of y is {y}");
}

//functions take parameters defining each parameter type
fn another_function(x: i32) {
    println!("The value of x is: {x}");
}

fn expressions(){
    //expressions evalutate to a value
    //not having a semicolon at the end of an expressions returns that value from the expression
    let y = {
        let x = 3;
        x + 1
    };

    println!("The value of y is: {y}");
}

//functions have return types
fn five() -> i32 {
    5
}

fn plus_one(x: i32) -> i32 {
    x + 1
}