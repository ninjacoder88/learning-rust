fn main() {
    math();

    data_types();

    arrays();
}

fn math(){
    let sum = 5 + 10;
    println!("{sum}");

    let difference = 95.5 - 4.3;
    println!("{difference}");

    let product = 4 * 30;
    println!("{product}");

    let quotient = 56.7 / 32.2;
    println!("{quotient}");

    let truncated = -5 / 3;
    println!("{truncated}");

    let remainder = 43 % 5;
    println!("{remainder}");
}

fn data_types(){
    let bool_true = true;
    println!("{bool_true}");

    let bool_false: bool = false;
    println!("{bool_false}");

    let char_little_z = 'z';
    let char_big_z: char = 'Z';
    let heart_eyed_cat = '😻';

    //tuples!
    let tup_int_float_char: (i32, f64, char) = (500, 6.4, 'G');

    let tup_int_float_int = (500, 6.3, 1);

    //tuples can be deconstructed
    let (a, b, c) = tup_int_float_int;
    println!("THe value of b is: {b}");

    //tuples values can be accessed with indexes
    let tup_i32_f64_u8 = (500, 6.4, 1);
    let five_hundred = tup_i32_f64_u8.0;
    let six_point_four = tup_i32_f64_u8.1;
    let one = tup_i32_f64_u8.2;    
}

fn arrays(){
    let months = ["January", "February", "March", "April", "May", "June", "July",
    "August", "September", "October", "November", "December"];

    let array_int = [1,2,3,4,5];

    //arrays are a fixed size determined at compile time
    let array_explicit: [i32; 5] = [1, 2, 3, 4, 5];

    //fill an array with (some value); X times
    let array_repeat_1 = [3; 5];
    let array_repeat_2 = [3, 3, 3, 3, 3];

    //access values in array using indexes
    let first_element = array_explicit[0];
    let second_element = array_explicit[1];
}