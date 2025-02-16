// this is basically about the return of function in rust =D
// if the return is reference , it need to tag a lifetime to make sure when the
// variable is used, it is not moved ;

pub fn return_basic() {
    //println!("{}", get_msg(0));
    // func_return();
    println!("{}", func_para(|x| x * x, 10));
}

fn get_msg(mark: i32) -> &'static str {
    if mark == 0 {
        "eq 0"
    } else {
        "else"
    }
}

// if the function has a parameter or return :

fn func_return() {}

fn func_para(f: fn(i32) -> i32, x: i32) -> i32 {
    f(f(x))
}
