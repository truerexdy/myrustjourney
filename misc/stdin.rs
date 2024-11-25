use std::io;
fn main(){
    let mut uin = Default::default();
    io::stdin().read_line(&mut uin).expect("Failed to read the input");
    let num: i8 = uin.trim().parse().expect("Input not an integer.");
    println!("{}",num);
}