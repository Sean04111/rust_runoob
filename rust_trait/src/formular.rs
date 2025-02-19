//Clone , Copy , Debug , Partialeq


#[derive(Debug,Clone)]
enum Race {
    White,
    Yellow,
    Black,
}

impl PartialEq for Race {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Race::Yellow,Race::Yellow) => true,
            (Race::Black,Race::Black) => true,
            (Race::White,Race::White) => true,
            (_, _) => false,
        }
    }
}

#[derive(Debug,Clone)]
struct User {
    id:u32,
    name:String,
    race: Race,
}

impl PartialEq for User {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id && self.name == other.name && self.race == other.race
    }
}


pub fn formula_basic(){
    let user = User{
        id:3,
        name:"user".to_owned(),
        race:Race::Yellow,
    };
    println!("{:#?}", user);

    let user2 = user.clone();
    println!("{:#?}", user2);

    let mut user3 = user;
    println!("{:#?}", user3);
    // println!("{:#?}", user);
    user3.race = Race::White;
    println!("eq is {}", user2 == user3);
}