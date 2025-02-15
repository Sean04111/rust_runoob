// this basically about the loop and iteratior in rust =D
//loop:
//      loop is like be used to start a endless code , or you end it with break;
//      loop {
//
//          //code
//
//          break // stop here
//      }
//while:
//      you can start a loop code at certain situation
//      while condition {
//
//          //code
//          break // stop here
//      }
//
//for:
//      for is always used with iteratior
//      for item in iterable {
//          //code
//      }
//
//break & continue : ..
//

pub fn loop_iterator_basic() {
    //loop {
    //    println!("1 sec");
    //    std::thread::sleep(std::time::Duration::from_secs(1));
    // }
    //

    let arr = vec![1, 2, 3, 4];
    for elem in arr {
        println!("element is {}", elem);
    }

    for i in 0..=10 {
        if i % 2 == 0 {
            continue;
        }
        println!("i is {}", i);
    }

    let numbers = [1, 2, 3, 4, 5].to_vec();
    let iter_number: Vec<_> = numbers.iter().map(|&x| x * x).collect();
    println!("{:?}", iter_number);

    // closure
    let f = |a: String, b: &str| a + b;
    println!("hello + world = {}", f("hello".to_owned(), "world"));

    // iterator details
    let new_vec = [10, 9, 8, 7, 6].to_vec();
    let test_iter = new_vec.iter();
    let new_arr: Vec<_> = test_iter.map(|x| if *x > 7 { 100 } else { *x }).collect();
    println!("new_vec {:?}", new_arr);
}
