use std::io;

fn main(){
    println!("Welcome to temperature scale!!!");
    println!("Please seelct your option:");
    println!("1. Cel to Far\n2. Far to Cel");
    
    let stdin = io::stdin();
    
    let mut response = String::new();
    
    stdin.read_line(&mut response);
    
    let response = response.trim();
    
    println!("You have selected option: {response}");
    
    let mut cel = String::new() ;
    let mut far  = String::new() ;
    
    if response =="1"{
        println!("Please enter Temperature in Cel:");
        stdin.read_line(&mut cel);
        let cel:f64  = cel.trim().parse().unwrap();
        println!("The temperature in Far is:{}",cel*1.8+32.0);
    }
    else{
        println!("Please enter Temperature in Far:");
        stdin.read_line(&mut far );
        let far:f64  = far.trim().parse().unwrap();
        println!("The temperature in Cel is:{}",far*0.55-32.0);
        
    }
    
   
    
}