fn main(){
    let mut i: i8 = 0;
    loop{
        i += 1;
        println!("{}", i);
        if i==100{
            break;
        }
    }
}