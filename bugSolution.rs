fn main() {
    let mut x = 5;
    { // Create a new scope
        let y = &mut x; // y is a mutable reference to x
        *y = 10; // Modify x through y
    } // y's scope ends here
    { // Create another new scope
        let z = &mut x; // z is a mutable reference to x
        *z = 15; // Modify x through z
    } // z's scope ends here
    println!("x = {}", x); // This will print 15
}
