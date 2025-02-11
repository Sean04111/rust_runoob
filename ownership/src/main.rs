// this is basically about the ownership in rust
// which is mentioned in the types section =D

fn main() {
    ownership();
}

fn ownership() {
    // copy : if the type is basic types such as int, char, float
    // the compiler will execute a copy operation
    // but if the type is like String Struct etc (refenrence types ?)
    // the compiler will hand the ownership to the new variable
    // you can use .clone() to deep copy the variable

    // basic types
    let a1 = 1;
    let a2 = a1;
    println!("a1 value is {}", a1);

    // refenrence types
    let str1 = String::from("str1");
    // let str2 = str1;
    // println!("str1 value is {}", str1); not ok cause this is a borrow of moved vlaue (str1 here
    // has been moved)
    // funciton stack pushing is ths same !
    //my_print(str1);

    //println!("str1 value is {}", str1); // not ok cause this is a borrow of moved vlaue (str1 here

    //println!("get_length: {}", get_length(str1));
    // println!("str1 value is {}", str1); // not ok
    let str1 = "hello";
    println!("get fisrt word is {}", get_first_word(str1));
    println!("get str1 value is {}", str1);

    let my_str = String::from("my_str");
    my_print_with_borrow(&my_str);
    println!("my_str value is {}", my_str);
}

fn my_print(str: String) {
    println!("printing {}", str)
    // the variable str moved here
}

fn get_length(str: String) -> usize {
    str.len()
    // the variable str moved here
}
// the funciton stack works fine with a borrow
// so it's a little different in rust and golang:
// in golang : push a variable into the funciton stack without refenrence , the compiler will new a
// variable and copy the value of the original variable push in ;
// in rust : you push a variable into the stack of a funciton , the compiler will hand the
// ownership to this funciton and the variable will be moved as the stack pop out
//
//
// so i think there is no such value copy or deep copy problem in rust ?
fn my_print_with_borrow(s: &String) {
    println!("printing with borrow {}", s);
}
// errors
fn dangle() -> String {
    "hello".to_owned()
}

// ‘ means lifetime here
fn dangle_static() -> &'static str {
    "hello"
}

// & means a borrow
fn get_first_word(s: &str) -> &str {
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }
    &s[..]
}
