fn main() {
    
}

fn loop_loop(){
    let mut counter = 0;

    //loop will run forever until exited, think while(true)
    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };

    println!("The result is {result}");
}

fn loop_labels(){
    let mut count = 0;
    'counting_up: loop {
        println!("count = {count}");
        let mut remaining = 10;

        loop {
            println!("remaining = {remaining}");
            if remaining == 9 {
                break;
            }
            if count == 2 {
                break 'counting_up;
            }
            remaing -= 1;
        }

        count += 1;
    }

    println!("end count = {count}");
}

fn while_loop(){
    let mut number = 3;

    while number != 0 {
        println!("{number}!");

        number -= 1;
    }

    println!("liftoff!!");
}

fn for_loop(){
    let a = [10, 20, 30, 40, 50];

    for element in a {
        println!("the value is {element}");
    }
}

fn for_loop_range(){
    for number in (1..4).rev() {
        println!("{number}!");
    }

    println!("liftoff!");
}
