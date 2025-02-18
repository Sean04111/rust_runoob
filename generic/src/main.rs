// this is basically about the generic in rust =D 


//generically struct
struct Point<T> {
    x:T,
    y:T,
}

impl <T> Point<T> {
    fn new(x:T, y:T) -> Point<T>{
        Point{x,y}
    }
}


//generically function
fn swap<T>(a:T, b:T) -> (T,T) {
    (b,a)
}


fn main() {
    println!("result is {:?}", swap::<i32>(1,2));
    
    let p1 = Point::new(1,2);
}
