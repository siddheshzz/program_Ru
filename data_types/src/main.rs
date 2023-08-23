fn main() {
    //ARRAYS

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


    //VECTORS
    //vectors are arrays of variable size
    //Two ways to create:
    // 1 Create object of Vec like Vec::new ()
    // 2 Create macro vec!

    let vecPrimes : Vec<i32> = Vec::new();
    let mut vecPrimes = vec![2,3,5];

    println!("{:?}",vecPrimes);

    vecPrimes.push(7);
    println!("{:?}",vecPrimes);
    vecPrimes.remove(2);
    println!("{:?}",vecPrimes);

    let mut vecNumbers = vec![2;10];
    println!("{:?}",vecNumbers);

    const vecDEFAULT: bool = true;

    let values = vec![vecDEFAULT;8];
    println!("{:?}",values);

    //update element

    vecNumbers[5] = 8;
    println!("{:?}",vecNumbers);

    for number in vecNumbers.iter(){
        println!("{:?}",number*number);
    }



}
