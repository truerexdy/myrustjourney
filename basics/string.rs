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


/*
1. &str is immutable, String is mutable
2. &str is called String slice
3. string slice is borrowed, String is owned
4. string slice is heap or stack allocated, String is heap allocated

&str is owned by the program, meaning it can be destroyed only
on termination of the program
*/

/*
Must know things about String:
1. Creating a String
    String::new();
    String::from("string literal");
    "string literal".to_string();
2. Push
    .push_str("string slice or literal");
    .push('c:char');
3. Length and Capacity
    .len();
    .capacity();
    .reserve(no of bytes);
    .shrink_to_fit();
4. Manipulation
    .trim();
    .to_lowercase();
    .to_uppercase();
5. Checking
    .contains("slice or literal");
    .starts_with();
    .ends_with();
6. Remove
    .clear();
    .remove(index);
7. Concatenation
    str1 + str2
8. Convert to Slice
    .as_str();
9. Spliting
    Vec<&str> = s.split('split char').collect();
    .split() splits
    .collect() creates an iterator that returns a Vec<&str>
10. Replacing
    .replace("old", "new");
*/