fn main(){
    let a = 6;

    let add_a = | x | a+x;
    // Here the closure add_a captured the variable a from the surrounding scope
    let b = add_a(8);
    println!("{}", b);
}