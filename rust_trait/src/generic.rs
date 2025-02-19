

trait Overview {
    fn overview(&self) ->String{
        String::from("Course")
    }
}

trait Another {
    fn hell(&self) {
        println!("welcome to hell");
    }
}

struct Course {
    headline: String,
    author:String,
}

impl Overview for Course{}

impl Another for Course{}

struct AnotherCourse {
    headline: String,
    author:String,
}

impl Overview for AnotherCourse{}



fn call_overview(item: &impl Overview) {
    println!("Overview {}", item.overview());
}

fn call_overview_generic<T:Overview>(item:&T){
    println!("Overview {}", item.overview());
}

fn call_overviewT(item1: & impl Overview, item2:&impl Overview){
    println!("Overview 1 is {}", item1.overview());
    println!("Overview 2 is {}", item2.overview());
}

fn call_overviewTT<T:Overview>(item1:&T, item2:&T){
    println!("Overview 1 is {}", item1.overview());
    println!("Overview 2 is {}", item2.overview());
}

fn call_mul_bind(item: &(impl Overview+Another)){
    println!("Overview is {}", item.overview());
    item.hell();
}

fn call_mul_bind_plus<T:Overview+Another>(item:&T){
    println!("Overview is {}", item.overview());
    item.hell();
}

fn call_mul_bind_with<T>(item:&T) where T: Overview + Another{
    println!("Overview is {}", item.overview());
    item.hell();
}

pub fn generic_basic(){
    let c1 = Course{headline:"ff".to_owned(),author:"yy".to_owned()};
    let c2 = Course{headline:"ff".to_owned(),author:"yz".to_owned()};
    let c3 =AnotherCourse{headline:"f1".to_owned(),author:"y1".to_owned()};
    call_overview(&c1);
    call_overview(&c2);

    call_overviewT(&c1,&c3);
    // call_overviewTT(&c1, &c3); // not ok cause in generic the Type (T specifically) should be the same
    call_overviewTT(&c1, &c2); // ok cause of the same type
    
    call_mul_bind_with(&c1);
}

