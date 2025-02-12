use std::vec;

// this is basically about the stack and heap in rust =D
//(stack and heap are not adta structure but the runtime part sepecificaly)
// 1. stack
//      stack will store the variables in the order got them , and delete them in the conversed
//      order;
//      the namespace of a function is on the stack;
//      all data in stack should have a unchangeable value and size
//
//   types:
//          basic types
//          tuple and array
//          struct(without String etc ) and enum
// 2. heap
//      heap is kind of disorder than stack , data in heap can have changeable value and size
//   types:
//          box rc
//          String / Vec etc
// basically types on stack will use copy( exclude struct ,struct will defaultly use move) , on heap will use move (this is the same principle in mut )
// if you do need to use copy for heap-based variables , you can impl a Copy trait (sth like
// interface...) ,or use the Clone function (after impl the Clone trait);
//
//
//Box pointer
//      you can allocate a variables on heap by using box pointer ;
//
//
mod box_test;

fn main() {
    //box_test::box_basic();
    copy_move();
}

fn copy_move() {
    //copy & move
    let x = vec![1, 2, 3, 4];
    let y = x.clone();
    println!("{:?}", y);
    println!("{:?}", x);

    let book1 = Book {
        page: 100,
        rating: 1.54,
    };
    let book2 = book1;
    // println!("{}", book1.page);
}

struct Book {
    page: i32,
    rating: f64,
}
