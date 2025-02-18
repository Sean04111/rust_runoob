pub fn lifetime_basic() {
    println!("{}", longest("hello", "world"));

    let s1 = "hello world";
    {
        let s2 = "world";
        println!("longger is {}", longest(s1, s2));
    }
}

fn life_no_need(s: &'static str, s1: &str) -> &'static str {
    s
}

// so  this needs the parameter s1, s2 have the same lifetime, if they don't , the compiler will
// panic
fn longest<'a>(s1: &'a str, s2: &'a str) -> &'a str {
    if s1.len() > s2.len() {
        s1
    } else {
        s2
    }
}

fn longest_v2<'a, 'b, 'out>(s1: &'a str, s2: &'b str) -> &'out str
where
    'a: 'out,
    'b: 'out,
{
    if s1.len() > s2.len() {
        s1
    } else {
        s2
    }
}

struct MyString<'a> {
    text: &'a str,
}

impl<'a> MyString<'a> {
    fn get_length(&self) -> usize {
        self.text.len()
    }
}

struct StringHolder {
    data: String,
}

impl StringHolder {
    fn get_length(&self) -> usize {
        self.data.len()
    }

    fn get_borrow<'a>(&'a self) -> &'a String {
        &self.data
    }
    
    fn get_ref(&self) -> &String{
        &self.data
    }
}
