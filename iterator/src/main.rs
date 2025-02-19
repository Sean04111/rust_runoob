
fn sum_with_loop(arr: &[i32]) -> i32 {
    let mut sum = 0;
    for item in arr {
        sum += *item;
    }
    sum
}

fn sum_with_iter(arr: &[i32]) -> i32 {
    arr.iter().sum()
}

fn main() {
    let arr = [1,2,3,4];
    println!("sum is {}", sum_with_iter(&arr));
}