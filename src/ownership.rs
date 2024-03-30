fn main(){
    let s = String::from("hello");
    takes_ownership(s);

    let x = 5;
    makes_copy(x);

    let s1 = gives_ownership();
    let s2 = String::from("hello");
    let s3 = takes_and_gives_back(s2);

    let s4 = String::from("hello");
    let (s5, len) = calculate_length(s4);
}

fn scope_and_drop(){
    let mut s = String::from("hello");

    s.push_str(", world");

    println!("{}", str);
}// s is dropped because it goes out of scope

fn copy_with_value_types(){
    let x = 5;
    let y = x; //the value in x is copied and bound to y
} // x and y are dropped

fn strings(){
    let s1 = String::rom("hello");
    let s2 = s1; //s1 is now invalid
}

fn deep_copy(){
    let s1 = String::from("hello");
    let s2 = s1.clone(); //s1 is still valid because clone is a deep copy
}

fn takes_ownership(some_string: String) {
    println!("{some_string}");
}

fn makes_copy(some_integer: i32){
    println!("{some_integer}");
}

fn gives_ownership() -> String {
    let some_string = String::from("hello");
    some_string;
}

fn takes_and_gives_back(a_string: String) -> String {
    a_string;
}

fn calculate_length(s: String) -> (String, usize) {
    let length = s.len();
    (s, length);
}