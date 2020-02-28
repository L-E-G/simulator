use std::fs::*;
use std::io::*;


fn main() {
    let addr = "54";
    let ret: String = get(addr.to_string());
    println!("{}", ret);
}

fn get(addr: String) -> String{

    let filename = "mem.txt";
    // Open the file in read-only mode (ignoring errors).
    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);

    // Read the file line by line using the lines() iterator from std::io::BufRead.
    for (index, line) in reader.lines().enumerate() {
        let line = line.unwrap(); // Ignore errors.
        let data: Vec<&str> = line.split(" ").collect();
        if addr == data[1].to_string(){
            return data[0].to_string();
        }
    }
    let message = "Unable to find your data";
    return message.to_string();

}