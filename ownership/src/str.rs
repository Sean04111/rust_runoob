//this is mainly about &str and String type in rust =D

//String is a changeable string type allocated in heap

//&str is a reference/borrow of a string slice , unchangeable, allocated in stack

// String has an ownership while &str does't

// here is some tips on choose String or &str
//
// in Struct, use String for its ownership and safety (but if you want to set the &str in struct ,
// you must set it's lifetime !)
// in function argument , usr &str if you don't want to hand the ownership

pub fn str_basic() {
    // use "".to_owned / to_string() to transfer &str to String
    let name = String::from("value name");
    let value = "value".to_owned();
    let value2 = "vlaue".to_string();

    let new_name = name.replace("name", "new_name");

    //in struct

    let color = "black".to_string();
    let name = "Jhon".to_string();
    let people = Person {
        name: name,
        color: color,
        age: 89,
    };

    let new_name = "Sean";
    let new_color = "white";
    let people_new = Person {
        name: new_name.to_string(),
        color: new_color.to_string(), // can't use &str
        age: 89,
    };
    let abc = "abc".to_string();
    print_str("abc");
    print_str(&abc);
    print_string(&abc);

    // print_string("abc"); not ok
}

struct Person {
    name: String,
    color: String,
    age: i32,
}

// 'a means the lifetime
struct Person2<'a> {
    name: &'a str,
    color: &'a str,
    age: i32,
}

// for function

// args can be &String, &str
fn print_str(s: &str) {
    println!("value {}", s);
}

// args can only be &String
fn print_string(s: &String) {
    println!("value {}", s)
}
