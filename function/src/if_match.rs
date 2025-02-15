// this is basically about the if , match and execution flow like that in rust =D

//  if : if is more readable than match , but match will be more
//
pub fn if_match_basic() {
    if_basic();
}

fn if_basic() {
    let a = 90;
    if a == 100 {
        println!("full score ");
    } else if a > 60 {
        println!("ok");
    } else {
        println!("dismiss");
    }

    let msg = if a > 0 { "attended" } else { "unpresent" };
    println!("{msg}");

    // match
    //
    let num = 100;
    match num {
        10..20 => println!("10-20"),
        20..=100 => println!("20-100"),
        _ => println!("else"),
    }

    match num {
        x if x > 10 => println!("num is > 10"),
        x if x > 20 => println!("num is > 20"),
        _ => println!("else"),
    }

    let msg = match num {
        x if x < 50 => "bad".to_owned(),
        x if x < 90 => "good".to_owned(),
        _ => "else".to_owned(),
    };

    print!("msg is {}", msg);
}
