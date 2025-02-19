use std::ops::Add;

#[derive(Debug)]
struct Point<T> {
    x:T,
    y:T,
}

// reload add operation for point struct
impl<T> Add for Point<T> where T: Add<Output = T>
{
    type Output = Point<T>;
    fn add(self, rhs: Self) -> Self::Output {
        Point{
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}




pub fn reload_basic(){
    let c1 = Point{x:1,y:0};
    let c2 = Point{x:2,y:1};
    
    let sum = c1 + c2 ;
    println!("{:?}", sum);
    
    let c3 = Point{x:1.91,y:3.93};
    let c4 = Point{x:8.91,y:4.13};
    let sum2 = c3+ c4;
    println!("{:?}", sum2);
}