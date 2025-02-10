use std::os::macos::raw::stat;

fn main() {
    println!("Hello, world!");
    // vars()
    //shadow_vars_int32()
    //const_test();
    static_test();
}

fn _vars() {
    let x: i32 = 5; // claim a int32

    let z = 3.9;
    let y = z as i32; // to transfer a var to another Type;

    println!("val:x = {}", x);
    println!("val:y = {y}");
}

//  some vars are unchangeable in rust for safe
fn un_changeable_vars() {
    // not used var should have the _ prefix to prevent warning message while compiling
    let _nice_count: i32 = 100;
    let mut _nice_number: i64 = 54;
    // nice_count = 100; not ok cause nice_count is not mut var
    _nice_number = 64;
}

// revalue a var is not revalue but shadow the formal value
fn shadow_vars_int32() {
    let x: i32 = 5;
    // new namespace
    {
        let x = 10;
        println!("inner x = {}", x);
        // the inner x is destroyed
    }

    println!("outer x = {}", x);

    let mut x = "hello"; // this overwrites the var in the same namespace
    x = "this";
    println!("New x = {}", x);
}

// here is basically about the const and static variables in rust =D

// const must confirm the type value before compile
// const in rust is diffrent from macro in c-lang
// const variables better be uppercased and use _ as seperation
// consts are only avaliable in the block it was claimed

// static can revlue with unsafe
// unsafe is not recommended cause it is not rust-style
// static are avaliable during the program

fn const_test() {
    //const
    const SECOND_HOUR: usize = 3_600;
    const SECOND_DAY: usize = 24 * SECOND_HOUR;
    {
        const SE: usize = 1_000;
    }
    println!("second a day : {}", SECOND_DAY);
    // println!("SE:{}",SE); not ok cause the const SE is only avaliable in the namespace where the
    // const is declared;
}

static MY_STATIC: i32 = 32;
static mut MY_MUT_STATIC: i32 = 32;

fn static_test() {
    //static
    // MY_STATIC = 100; not ok cause the staitc variables are not allowed to revalue
    println!("my static is : {}", MY_STATIC);

    unsafe {
        MY_MUT_STATIC = 100;
        println!("my mut static is : {}", MY_MUT_STATIC);
    }
}
