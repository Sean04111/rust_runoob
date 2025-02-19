use std::os::unix::fs::DirBuilderExt;

trait Driver {
    fn driver (&self);
}

struct Car;
impl Driver for Car {
    fn driver(&self) {
        println!("Car driver");
    }
}

struct SUV;
impl Driver for SUV {
    fn driver(&self) {
        println!("SUV driver");
    }
}

fn road (vehicle: &impl Driver){
    vehicle.driver();
}

trait Queue {
    fn len(&self) -> usize;
    fn push_back(&mut self , n:i32);
    fn pop_front(&mut self) -> Option<i32>;
}

trait Deque:Queue {
    fn push_front(&mut self, n:i32);
    fn pop_back(&mut self) -> Option<i32>;
}

pub fn polymorphism_basic(){
    let c = Car;
    let s = SUV;
    road(&c);
    road(&s);
}

