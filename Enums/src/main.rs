use crate::Colors::Green;

fn main() {
     let my_color = Colors::Red;
     println!("color: {:?}", my_color);
     let my_color = Green;
     println!("color: {:?}", my_color);

     let person = Person::Name(String::from("John"));
     println!("person: {:?}", person);
     
}
#[derive(Debug)]
enum Colors{
    Red, Green,Blue
}
#[derive(Debug)]
enum Person{
    Name(String),
    Surname(String),
    Age(u32)
}
