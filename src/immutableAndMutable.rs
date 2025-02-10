fn main () {
    let x = 5; //Immutable
    let mut y = 10; //Mutable
    println!("x: {}, y: {}", y, x);

    y = 20; //Allowed because it's mutable
    println!("updated y: {}", y);
}