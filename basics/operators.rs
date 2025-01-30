fn main(){
    // Arithmatic
    println!("5+5={}",5+5);
    println!("7+2={}",7+2);
    println!("9*6={}", 9*6);
    println!("5/3={}",5/3);
    println!("10%6={}",10%6);
    
    //Bitwise
    let a:i32 = 5;
    let b:i32 = 9;
    println!("\na = 5 = {:b} \n b = 9 = {:b}", a, b);
    println!("a&b = {:b}", a&b);
    println!("a|b = {:b}", a|b);
    println!("a^b = {:b}", a^b);
    println!("a<<2 = {:b}", a<<2);
    println!("a>>1 = {:b}", a>>1);

    //Logical
    let a:bool = true;
    let b:bool = false;
    println!("a = {} \n b = {}", a, b);
    println!("a&&b = {}", a&&b);
    println!("a||b = {}", a||b);
    println!("!b = {}", !b);

    //Assignment
    /* Not to mention, too basic */

    //Relational
    /* Same as above */

    //Range
    // ..
    for i in 5..10{
        println!("{}", i);
    }

    // ..=
    for i in 5..=10{
        println!("{}", i);
    }

    // Type casting
    let a:i32 = 5;
    let b:f64 = a as f64;
    println!("a = {} is an i32\nb={} is a f64", a, b);

    // Borrowing
    /* Requires use of functions */

    // Dereferencing !!Unsafe, use of raw pointer is unsafe. Borrow checker doen't check raw_ptr!!
    unsafe{
        let a:*const f64 = &b;
        println!("a is of type *f64 and *a = {:.2} and a = {:p}", *a, a);
    }
}