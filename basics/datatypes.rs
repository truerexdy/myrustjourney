fn main(){
    /*Primitive datatypes
    1. Integer
    2. Float
    3. Boolean
    4. Character*/

    let x:i8 = -5;
    println!("{x} is a i8");
    /*Signed Integer types:
    i8, i16, i32, i64, i128*/

    let y:u8 = 8;
    println!{"{y} is a u8"};
    /*Unsigned integer types:
    u8, u16, u32, u64, u128*/

    let z:f64 = 69.69;
    println!("{z} is a f64");
    /*Float types:
    f64, f128*/

    let status:bool = true;
    println!("{status} is a bool");
    /*bool is the keyword. true and false*/

    let letter:char = 'C';
    println!("{letter} is a char");
    /*here char is a UTF-8 char not a ASCII char*/


    /*Compound Types
    1. Tuples
    2. Arrays
    3. Slices*/

    let ipv4:(u8,u8,u8,u8) = (192,168,1,100);
    println!("ipv4 is a tuple");
    /*Tuples can have values of different types
    Tuples are immutable by default*/

    /*Tuples can be Destructured in the following way:*/
    let (a,b,c,d) = ipv4;
    println!("{a}.{b}.{c}.{d}");

    let arr[u8, 5] = [0,1,2,3,4];
    //Initializing an array
    let nullarr[u8,10] = [0; 10];
    // Initializing the array with single value

    /*Slices are just references to an array slice*/
}