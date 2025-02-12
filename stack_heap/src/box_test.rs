struct Point {
    x: i32,
    y: i32,
}

pub fn box_basic() {
    let boxed_point = Box::new(Point { x: 10, y: 10 });
    // boxed_point is like a pointer points to the heap
    println!("point is x: {}, y: {}", boxed_point.x, boxed_point.y);
    println!("the boxed_point addres is {:p}", boxed_point);
    let mut boxed_int = Box::new(2);
    println!("{}", *boxed_int);
    *boxed_int += 100;
    println!("{}", *boxed_int);
}
