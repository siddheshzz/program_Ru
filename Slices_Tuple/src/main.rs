fn main() {
    //SLICE
    //Slice is a pointer to a block of memory
    let numbers = [1,2,3,4,5];
    let slice = &numbers[1..4];
    //lower bound included upper bound excluded
    println!("{:?}", slice);
    //Size is determined at run time
    //Can be used in arrays, vectors and Strings
    //Indexed
    //Mutable slices allow us to change the value

    let mut colors = ["red","green","blue","pink"];
    println!("{:?}", colors);
    update_colors(&mut colors[2..4]);
    println!("{:?}", colors);



    //TUPLE
    //A collection of values of various types

    let mut person:(&str, i64,bool) = ("John",27,true);
    println!("{:?}", person);

    // max size of a tupple can be 12 if we go on adding futher then we will get an error

    //Accessing element tuple
    println!("{}",person.0);

    //Updating elemt in tuple
    person.0 = "Ram";
    println!("{}",person.0);

    //Destructing a tuple
    let (name, age, employed) = person;
    println!("{},{},{}",name,age,employed);

    



    
}

fn update_colors(colors_slice: &mut [&str]){
    colors_slice[0] = "yellow";
    colors_slice[1] = "black";

}
