fn main() {

    //CLOSURE
    //|    |  enclosed in these and is similar to lambda

    let a = |a: i32| a+1;
    println!("{}", a(6));

    let b = |b: i32| -> i32{
        let c = b+1;
        c
    };
    println!("{}", b(4));

    //a closure can be generic
    // = |x| 

    let gen = |x| println!("{}", x);
    gen(3);
    
    // gen(true);
    // this above will not work because of some memory issue which would be understood in future advance conecpts


    //HIGHER ORDER FUNCTIONS
    //Fucntion that take another function as a paramater
    let square = |a :i32| a*a ;
    apply(square,6);

    //calculate the sum of all the squares less than 500
    //normal approach
    let mut a = 1;
    let limit = 500;
    let mut add = 0;
    loop{
        add = add + (a*a);
        a = a+1;

        if a*a>=500{
            break;
        }
    }
    println!("{0}-{1}",add,a);

    //higher order approach
    let sum2 = (0..).map(|x| x*x)
                .take_while(|&x| x<=limit)
                .filter(|x| is_even(*x))
                .fold(0,|sum,x| sum+x);
    
    println!("{}",sum2)





}
fn is_even(x:i32) -> bool{
    x % 2 == 0
}

fn apply(f: fn(i32) -> i32, a:i32){
    println!("Result {}", f(a));
}

