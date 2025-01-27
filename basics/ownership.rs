/*Ownership is a key feature of Rust and one needs to understand it in detail*/

/*
fn print_my_string(a:String){
    println!("{}",a);
}

fn main(){
    let a : String = String::from("Hello");
    print_my_string(a);
    println!("{}",a);
}

    above code does not compile because, print_my_string takes ownership of a and does not return it
    back to main so in line 11 the print macro does not have access to a
*/

fn print_my_string(a:&String){
    println!("{}",a);
}

fn main(){
    let a : String = String::from("Hello");
    print_my_string(&a);
    println!("{}",a);
}

// This will work since it's just a immutable reference