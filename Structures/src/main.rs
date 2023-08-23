fn main() {
    //Structures
    let emp = Employee{
        name: String::from("John"),
        company: String::from("Google"),
        age : 30
    };
    println!("{:?}", emp);
    println!("{}",emp.name);
    println!("{}",emp.fn_details());
    //Static function call
    println!("{}", Employee::static_fn_detail());

}
//added below anotation so as to tell the compiler about our Stucture
//something like this is not standard so we need to inform complier so that it can then print accordingly

#[derive(Debug)]
struct Employee{
    name: String,
    company: String,
    age:u32
}

//Adding method to a structure
impl Employee {
    fn fn_details(&self) -> String {
        return format!("{} is {}. He works in {}", self.name,self.age,self.company);
    }
    //A structure can have static method
    fn static_fn_detail() -> String {
        String::from("Details of a Person")
    }
}