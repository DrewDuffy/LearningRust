use std::io;

fn main() {
    let mut fah = String::new();
    loop { 
        println!("Please input degrees in fahrenheit to convert to Celcius");
        io::stdin().read_line(&mut fah).expect("Failed to read line");
        let x: i32 = fah.trim().parse().expect("Input not an integer");

        let celcius : i32 = ({x} - 32) * 5 / 9; 

        println!("{x} degrees fahrenheit converts to {celcius} degrees celcius");
        break;
    }}
