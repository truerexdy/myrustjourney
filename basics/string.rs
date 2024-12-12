fn main(){
    /*String a complex datatype in Rust
    1. String
    2. &str*/

    let name:String = String::from("rexdy");
    /*String is defined in the alloc::string module of the
    Rust Standard Library.
    String is just a wrapper around Vec<u8>
    Vector because it's a dynamic array of u8, here u8 because UTF-8(upto 8 bit)*/
    println!("{}", name);
    /*String datatype ownes the data*/

    let code:&str = "this is an &str string";
    /*&str is an immutable reference to a string,
    immutable meaning it has only read access
    reference meaning it does not own the data*/
    println!("{}", code);
    /*in the above intialization nobody owns the data, code is just an immutable reference
    it will be stored in the read only part of the program*/

    let strslice:&str = &name[0..=2];
    println!("{} is an immutable reference to a slice of a string", strslice);

    /*Since String is a wrapper around Vec<u8>, UTF-8 char are converted or
    encoded into raw bytes and these are stored in String
    then while printing or doing any operation that involves char, the raw bytes are
    then decoded back to UTF-8 scalar values*/
}