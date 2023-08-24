//using and external module rand which first needs to be configured under dependencies in toml file
use rand::Rng;
use crate::Suit::{Heart,Club,Diamond,Spade};

fn main() {
    //If statement
    let mut rng = rand::thread_rng();
    let num:i32 = rng.gen_range(0,11);

    if num >5{
        println!("Number {} is greater than or equal to 5",num);
    }else if num ==5{
        println!("Number {} is  5",num)
    }
    else{
        println!("Number {} is smaller than 5",num);
        
    }

    let res:bool = if num >=5{true} else {false};

    println!("{}",res);

    //Match
    //Match is similar to when or switch statement

    print_choice(Heart);
    print_choice(Club);
    print_choice(Spade);
    print_choice(Diamond);

    country(44);
    country(555);
    country(-15);



    //Pattern Matching

    for i in 0..15{
        println!("{}, I have {} oranges", i,get_oranges(i));
    }

    let point = (0,0);
    match point{
        (0,0) => println!("origin"),
        (x,0) => println!("x axis ({},0)",x),
        (0,y) => println!("y axis (0,{})",y),
        (x,y) => println!("({},{})",x,y)
    }


    //For Loop
    for i in 1..11{
        println!("{0} * {0} = {1}",i,i*i);
    }

    let pets = ["cat","dog","cow","sparow","crow"];
    for pet in pets.iter(){
        if pet == &"cat"{
            println!("{0} catt",pet);
        }
        println!("{}",pet);
    }

    for (i,j) in (1..11).enumerate(){
        println!("{0} * {0} = {1}",i+1,j*j);
    }

    get_squares(323);
    get_cubes(743)



    

}

fn country(code:i32){
    let country = match code {
        44 => "UK",
        34=> "United States",
        1..=999 => "Canada",
        _=>"invalid"
    };
    println!("Country: {}", country);


}


enum Suit{
    Heart,
    Spade,
    Club,
    Diamond
}

fn print_choice(choice: Suit){
    match choice {
        Heart => {println!("\u{2665}")}
        Spade => {println!("\u{2668}")}
        Club => {println!("\u{2663}")}
        Diamond=> {println!("\u{2666}")}
    }
}

//Pattern matching match statement

fn get_oranges(amount:i32) -> &'static str {
    return match amount {
        0 => "no",
        1 | 2 => "one or two",
        3..=7 => "a few",
        _ if amount%2 ==0 => "an even amount of",
        _ => "lots of"
    }

}

//while loop

fn get_squares(limit:i32){
    let mut x:i32 = 1;
    while x*x <limit{
        println!("{0}*{0} = {1}",x,x*x);
        x = x+1;
    }
}

fn get_cubes(limit:i32){
    let mut x:i32 = 1;
    loop {
        println!("{0}*{0}*{0} = {1}",x,x*x*x);
        x = x+1;

        if x *x*x >limit{
            break;
        } 
    }
}