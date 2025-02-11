// this is basically about enum type in rust =D

enum Color {
    Red,
    Yellow,
    Blue,
    Black,
}

fn print_color(my_color: Color) {
    match my_color {
        Color::Red => println!("Red"),
        Color::Yellow => println!("Yellow"),
        Color::Blue => println!("Blue"),
        _ => (),
    }
    {}
}

enum BuildingLocation {
    Number(i32),
    Name(String),
    Unknown,
}

impl BuildingLocation {
    fn print_location(&self) {
        match self {
            BuildingLocation::Number(c) => println!("location as number is {}", c),
            BuildingLocation::Name(c) => println!("location as name is {}", c),
            BuildingLocation::Unknown => println!("location Unknown"),
        }
    }
}

fn main() {
    // print_color(Color::Black);

    let house1 = BuildingLocation::Name("USA".to_string());
    house1.print_location();

    let house2 = BuildingLocation::Number(1);
    house2.print_location();

    let house3 = BuildingLocation::Unknown;
    house3.print_location();
}
