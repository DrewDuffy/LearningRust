fn main() {
    let mut count = 10000;
    loop {
//        println!("{}", count);
        count -=1;
        if count == 0 {
            println!("Blast off!");
            break
        }
    }
}
