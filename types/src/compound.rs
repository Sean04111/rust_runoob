// this is basically about somt compound type like arrays and tuples =D;
// all compound types length are fixed;
// 1. arrays elements -> same typs;
// 2. tuples elements -> different typs;
pub fn compound() {
    arrays();
    tuples();
    ownership();
}

// basic infomation about arrays
fn arrays() {
    let mut arr = [11, 12, 34];
    arr[0] = 100;
    println!("arr 0 is {}, length is {}", arr[0], arr.len());

    for i in 0..arr.len() {
        println!("elements are {} in range type 1", arr[i]);
    }

    for ele in arr {
        println!("elements are {} in range type 2", ele)
    }

    let mut arr2 = [2; 3];
}

// basic infomation about tuples
fn tuples() {
    let tup = (0, "hi", 3.4);
    println!("tup elements {}, {},{}", tup.0, tup.1, tup.2);

    // no len for tup
    // tup.len();

    let mut tup2 = (0, "fuck", 3.78);
    tup2.1 = "ok";
    // can not change the type of the elements;

    println!("tup2 elements {}, {},{}", tup2.0, tup2.1, tup2.2);
    // ()
    let tup3 = ();
    println!("tup3 {:?}", tup3);
}

// basic infomation about ownership
fn ownership() {
    //copy clone
    let int_item = 1;
    let mut int_ownership = int_item;

    int_ownership = 100;
    println!("int_item is {}", int_item);
    println!("int_ownership is {}", int_ownership);

    // move ownership for type String, struct
    let str_item = String::from("str_item");
    let mut str_ownership = str_item;

    str_ownership = String::from("str_ownership");

    // println!("str_item is {}", str_item); the variable str_item is not exist cause the ownership
    // is transfer to the variable str_ownership;
    println!("str_ownership is {}", str_ownership);
}
