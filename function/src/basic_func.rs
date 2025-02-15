// this is basically about the basic function in rust =D
//

fn add(x: i32, y: i32) -> i32 {
    x + y
}

// x is value copy
fn change(mut x: i32) {
    x += 4;
}

fn modify(x: &mut i32) {
    *x += 3;
}

// use the default copy
#[derive(Copy, Clone)]
struct Point {
    x: i32,
    y: i32,
}

fn print_point(point: Point) {
    println!("point is {},{}", point.x, point.y);
}

pub fn function_basic() {
    let mut a = 1;
    let b = 2;
    let c = add(a, b);
    change(a);
    println!("{a}");

    modify(&mut a);
    println!("{a}");

    let p = Point { x: 1, y: 1 };
    print_point(p);
    println!("{}", p.x);

    let mut n = 1;
    let mut s = String::from("string");
    move_change_function(&mut n, &mut s);
    println!("n is {}", n);
    println!("s is {}", s);

    let mut p1 = Point { x: 1, y: 1 };

    move_point(&mut p1);

    println!("the point after move is {}, {}", p1.x, p1.y);
}

fn move_function(p1: &i32, p2: &String) {
    println!("p1 is {}", *p1);
    println!("p2 is {}", *p2);
}

fn move_change_function(p1: &mut i32, p2: &mut String) {
    *p1 += 100;
    *p2 += "fuck";
}

fn move_point(point: &mut Point) {
    (*point).x += 10;
    (*point.y += 10;
}
