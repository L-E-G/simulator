use std::fs::*;
use std::io::*;
use std::str::FromStr;


fn main() {
    let addr = "8";
    let data = "111111";
    set(data, addr);
}

fn set(data: &str, addr: &str) {

    let filename = "mem.txt";
    let mut file = File::open(filename).unwrap();
    let reader = BufReader::new(file);
    let mut file2 = File::create("mem2.txt").unwrap();
    let mut buffer = LineWriter::new(file2);

    let mut i = 0;
    for (index, line) in reader.lines().enumerate() {
        let line = line.unwrap();
        let val: Vec<&str> = line.split(" ").collect();
        
        let naddr: u32 = FromStr::from_str(addr).unwrap();
        let nval: u32 = FromStr::from_str(val[1]).unwrap();

        if naddr < nval && i==0{
            println!("Rob");
            buffer.write_all(format!("{} {}\n",data, addr).as_bytes());
            i = 1;
        }
        buffer.write_all(format!("{} {}\n",val[0], val[1]).as_bytes());
    }
    buffer.flush();
}