// this is mainly anout the variables type in rust =D
// 1. Integer types -> default i32(int32)
//      i8,i16,i32,i64,i128
// 2.Unsigned Integer typs
//      u8,u16,u32,u64,u128
// 3. Platform-Specific Integer type the size is depending the arch
//      usize, isize
// 4. Float types
//      f32,f64(recommended, unlesss you clearly know the bound!)
// 5. Boolean types
//      true, false
// 6.Character types
//      Unicode supported
//      use '' instead of ""
//

fn main() {
    basic();
}

fn basic() {
    let a1 = -125;
    let a2 = 0xFF;
    let a3 = 0o13;
    let a4 = 0b10;
    println!("a1 {a1}");
    println!("a2 {a2}");
    println!("a3 {a3}");
    println!("a4 {a4}");

    // max && min
    println!("u32 max {}", u32::MAX);
    println!("u32 min {}", u32::MIN);
    println!("i32 max {}", i32::MAX);
    println!("u32 min {}", u32::MIN);
    println!("usize max {}", usize::MAX);

    //byte size
    println!("isize is {} byte", std::mem::size_of::<isize>());
    println!("usize is {} byte", std::mem::size_of::<usize>());

    // Float
    let f1: f32 = 1.223;
    let f2: f64 = 42.4224;
    let n = f1 as i32;
    println!("Floats are : f1 {:.2}, f2{:.2}, i32 n: {}", f1, f2, n);

    //Boolean
    let _is_ok = true;
    let _not_ok = false;

    //Character
    let char_c = 'c';
    let emoji_c = '😗';
    println!("char_c is {}", char_c);
    println!("emoji_c is {}", emoji_c);
}
