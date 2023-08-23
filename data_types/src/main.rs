fn main() {
    let primes:[i32;5] = [2,3,5,7,11];
    let double:[f64;4] = [2.8,4.8,4.5,7.89];

    println!("{:?}", primes);
    println!("{:?}", double);

    //default value array
    let numbers:[i32;15] = [0;15];
    println!("{:?}",numbers);

    const DEFAULT:i32 = 3;
    let mut numbers :[i32;15] = [DEFAULT;15];
    println!("{:?}",numbers);

    //access element by index but first need to declare number mutable
    numbers[3] = 5;
    println!("{:?}",numbers);

    //iterator

    for number in numbers.iter() {
        println!("{}",number);
    }
}
