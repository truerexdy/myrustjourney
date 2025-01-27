use std::io;
// use keyword is used to import a module

fn main(){
    /*User output is very easy, just use the println!() macro.
    ! indicates that it's a macro*/

    println!("Hello Amma");

    /*{} is used as a place holder for variables.
    the variables can either be inserted directly into the {}
    or as a seperate argument(idk what it's called for a macro)
    to the macro, along with the string that contains the string*/

    let my_num:u64 = 69;
    println!("{my_num}");
    //or
    println!("{}", my_num);

    /*Console input is a little tricky in Rust
    std::io module is used for user input
    
    io::stdin().read_line(&mut ref)
    is the method*/

    /*In Rust, any and every exception must be handeled.
    Since, the user not entering anything during input can
    lead to undefined behaviour. This exception must be handeled.
    using .except()*/
    let mut uin = String::new();
    //uin is a mutable string
    io::stdin().read_line(&mut uin).expect("failed to read line from console");
    println!("This is what you have entered:\n{}", uin.trim());

    /*There's a lot going on in the above few linse:
    1. &mut uin is a mutable reference to uin, uin still owns the data
        &mut uin just borrows the data. In contrast to immutable references
        mutable reference allows the data to be modified
    2. :: is namespace resolution operator, namespace not as in C++
        here modules are used to manage code. So to resolve the 
        modules and it's members we use this operator.
            For starters: A module is just a folder containing
            .rs files whose code can be reused by importing
            a module. Not, to be confused with Crate,
            crate is an entire project or an entire compilation unit.
            whereas module is just a part of it that is used to mainly
            organise code
    3. . operator is used to access methods and members of a
        struct or a enum*/

    
    /*Now in order to take an integer as input, there's no direct way. Enery userinput is a string.
    So we must convert that userinput string to integer*/

    {
        let mut userinput:String = String::new();
        io::stdin().read_line(&mut userinput).expect("Error reading a string");
        let convnum:i32 = match userinput.trim().parse(){
            Ok(num) => num,
            Err(_) => {
                println!("Invalid Number");
                return;
            }
        };
        println!("{convnum}")
    }
}