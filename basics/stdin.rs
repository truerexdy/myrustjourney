use std::io;

fn main() {
    
    let mut buffer: String = String::new();

    match io::stdin().read_line(&mut buffer) {
    
        Ok(bytes) => {
            println!("{} bytes read.", bytes);
        }
        Err(error) => {
            println!("{}", error);
        }
    }
    
    println!("The string is \n{}", buffer);
}
