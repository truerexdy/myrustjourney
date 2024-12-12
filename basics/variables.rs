fn main(){
    // Variables in Rust are Immutable by Default
    let x:i8 = 114;
    // let keyword is used to declare variables
    // let identifier:datatype = value;

    // To make variables mutable we must explicitly use the mutable keyword
    let mut y:i8 = 114;
    println!("{y} is an mutable variable");

    // x = 120; will not compile
    y = 120; //will compile
    println!("{x} is an immutable variable");
    println!("value of y after mutating {y}");

    {
        // This is a inner scope
        let x:i8 = 110;
        println!("x = {x} of inner scope");

        // Even though x is declared and initialised in the outer scope, it is shadowed by the
        // declaration in the inner scope
        
        // {} are used to create a inner scope
    }

    //constants
    const PI:f64 = 3.14;
    //const are evaluated at compile time
    //don't occupy memory during runtime
    //can be declared in any scope
    println!("PI = {PI}");
}