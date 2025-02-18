pub fn result_basic() {
    match divide(1, 2) {
        Ok(number) => println!("result is {}", number),
        Err(err) => println!("error is {}", err),
    }

    match divide(1, 0) {
        Ok(number) => println!("result is {}", number),
        Err(err) => println!("error is {}", err),
    }

    let arr = [1, 2, 3, 4, 5, 6];
    match find_element(&arr, 4) {
        Some(index) => println!("found in {}", index),
        None => println!("not found"),
    }

    let vec = vec![1, 2, 3];
    println!("{}", vec[4]);
}

fn divide(a: i32, b: i32) -> Result<f64, String> {
    if b == 0 {
        return Err(String::from("can't divide zero"));
    }

    let a = a as f64;
    let b = b as f64;

    Ok(a / b)
}

fn find_element(array: &[i32], target: i32) -> Option<usize> {
    for (idx, elem) in array.iter().enumerate() {
        if (*elem) == target {
            return Some(idx);
        }
    }
    None
}
