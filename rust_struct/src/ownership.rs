// this is basically about the ownership on the struct in rust =D

struct Counter {
    number: i32,
}

impl Counter {
    fn new(number: i32) -> Counter {
        Counter { number: number }
    }

    fn print(&self) {
        println!("number is {}", self.number);
    }

    fn change(&mut self, new_number: i32) {
        self.number = new_number;
        println!("change successful !");
        self.print();
    }

    fn get(&self) -> i32 {
        self.number
    }

    fn give_up(self) {
        println!("give_up number : {}", self.number);
    }

    fn combine(c1: Counter, c2: Counter) -> Counter {
        Counter {
            number: c1.number + c2.number,
        }
    }
}

pub fn basic() {
    let mut c = Counter { number: 100 };
    c.print();

    c.change(200);

    println!("number got is {}", c.get());

    // to check var c still exist ?
    c.print();

    let c1 = Counter::new(0);
    let c2 = Counter::new(1);
    let c3 = Counter::combine(c1, c2);

    c3.print();
}
