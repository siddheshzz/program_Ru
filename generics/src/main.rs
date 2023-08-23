fn main() {
    let p1: Point<i32> = Point{X:2,Y:7};
    let p2: Point<f64> = Point{X:5.23,Y:56.54};
    println!("{:?}", p1);
    println!("{:?}",p2);

    let c1= Colors::Red("#f00");
    let c2 = Colors::Red(255);

    println!("{:?}", c1);
    println!("{:?}", c2);

    let p3 :Point2<i32,f64> = Point2{x:32,y:4.23};
    println!("{:?}", p3);
    
}
#[derive(Debug)]
struct Point<T>{
    X: T,
    Y: T,
}

#[derive(Debug)]
enum Colors<T>{
    Red(T),
    Green(T)
}
#[derive(Debug)]
struct Point2<T,V>{
    x: T,
    y: V
}