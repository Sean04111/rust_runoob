// this is basically about the struct in rust =D
//

mod ownership;

struct Point {
    x: i32,
    y: i32,
}
impl Point {
    fn distance(&self, other: &Point) -> f64 {
        let dx = (self.x - other.x) as f64;
        let dy = (self.y - other.y) as f64;
        (dx * dx + dy * dy).sqrt()
    }

    fn new(x: i32, y: i32) -> Point {
        Point { x, y }
    }

    const MyPI: f64 = 3.15;
}

enum Flavor {
    Spicy,
    Sweet,
    Fruity,
}

struct Drink {
    flavor: Flavor,
    price: f64,
}

fn print_drink(drink: &Drink) {
    match drink.flavor {
        Flavor::Spicy => println!("Spicy drink"),
        Flavor::Sweet => println!("Sweer drink"),
        Flavor::Fruity => println!("Fruity drink"),
        _ => println!("unknown flavor drink"),
    }

    println!("price is {}", drink.price);
}

impl Drink {
    const MAX_PRICE: f64 = 1.0;

    fn buy(&self) {
        if self.price > Drink::MAX_PRICE {
            println!("can't buy");
            return;
        }
        println!(" success buy");
        print_drink(self);
    }

    fn new(flavor: Flavor, price: f64) -> Drink {
        Drink {
            flavor: flavor,
            price: price,
        }
    }
}

fn main() {
    //    let pointA = Point { x: 10, y: 10 };

    //    let pointB = Point { x: 5, y: 5 };

    //    println!("points distance is {:.2}", pointA.distance(&pointB));

    //let drink1 = Drink {
    //    flavor: Flavor::Fruity,
    //    price: 0.78,
    //};

    //drink1.buy();

    //let drink2 = Drink::new(Flavor::Fruity, 0.98);
    //drink2.buy();
    ownership::basic();
}
