// trait object in rust =D
// we shall take it as interface


use crate::Person;

trait  Overview {
    fn overview(&self)->String{
        String::from("overview")
    }
}

struct Obj {

}

impl Overview for Obj{
    fn overview(&self) -> String {
        String::from("obj")
    }
}

// this is un mutable borrow of a trait object
fn call_obj(item: &impl Overview){
    println!("overview form {}", item.overview());
}

// Move Operation must be used with Box pointer, cause all trait object would be allocated on heap instead of stack
fn call_obj_box(item: Box<dyn Overview>) {
    println!("overview from {}", item.overview());
}

// another

trait Sale {
    fn amount (&self) -> f64;
}


struct Common(f64);


impl Sale for Common {
    fn amount(&self) -> f64 {
        self.0
    }
}

struct TenDiscount(f64);

impl Sale for TenDiscount {
    fn amount(&self) -> f64 {
        self.0 - 10.0
    }
}

struct TenPercentDiscount(f64);

impl Sale for TenPercentDiscount {
    fn amount(&self) -> f64 {
        self.0 * 0.9
    }
}

fn calculate(sales: &Vec<Box<dyn Sale>>) -> f64{
    sales.iter().map(|sale| {sale.amount()}).sum()
}


pub fn object_basic(){
    let a = Obj{};
    call_obj(&a);
    println!("{}",a.overview());

    let b_a = Box::new(Obj{});
    call_obj_box(b_a);
    // println!("{}",b_a.overview());

    let c:Box<dyn Sale> = Box::new(Common(100.0));
    let t1:Box<dyn Sale> = Box::new(TenDiscount(100.0));
    let t2:Box<dyn Sale>  = Box::new(TenPercentDiscount(100.0));

    // let sales : Vec<Box<dyn Sale>> = vec![c,t1,t2];
    let sales = vec![c, t1,t2];
    println!("total sum is {}",calculate(&sales));
    // println!("c is {}", c.0);
}




