mod object;
use object::object_basic;


trait Greeter {
    fn greet(&self);
    fn hello(){
        println!("hello");
    }
}

struct Person {
    name:String,
}

impl Greeter for Person{
    fn greet(&self) {
        println!("greet from {}", self.name);
    }
}


fn main() {
    // let sean = Person{name:"sean".to_owned()};
    // sean.greet();
    // Person::hello();
    object_basic();
}

