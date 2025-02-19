mod object;
mod generic;
mod reload;
mod polymorphsim;
mod formular;

use generic::generic_basic;
use object::object_basic;
use reload::reload_basic;
use polymorphsim::polymorphism_basic;
use formular::formula_basic;


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
    // object_basic();
    // generic_basic();
    // reload_basic();
    // polymorphism_basic();
    formula_basic();
}


