fn main () {
    // Mutable and Immutable
    let x1 = 5; //Immutable
    let mut y = 10; //Mutable
    println!("x: {}, y: {}", y, x1);

    y = 20; //Allowed because it's mutable
    println!("updated y: {}", y);
    
    // Shadowing
    let a: i32 = 42; // Explicit type 
    let b = 3.14; // F64 inferred
    let is_active: bool = true;
    let c = 'R';
    
    // Shadowing example 
    let x2 = 5; 
    let x2 = x2 + 1; // New x shadows old x
    println!("x2: {}", x2);
    
}
