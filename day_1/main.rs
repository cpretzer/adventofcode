// use std::env;
use std::io;
use std::io::prelude::*;
use std::fs::File;

fn main() -> io::Result<()> {
    // let args: Vec<String> = env::args().collect();

    // let filename = &args[1];

    // create a buffer to hold the file contents
    let mut buf = String::new();

    // open the file and read it
    let mut f = File::open("input.txt")?;
    f.read_to_string(&mut buf)?;

    // let mut cursor = Cursor::new(&mut buf);
    // // assert!(cursor.buffer().is_empty());

    // let mut num_bytes = cursor.read_line(&mut buf)
    //     .expect("reading from buffer won't fail");

    // while num_bytes > 0 {
    //     println!("{}", &mut buf);
    //     num_bytes = cursor.read_line(&mut buf)
    //         .expect("read from buffer won't fail");
    // }


    // create some variables and split the file contents by the newline
    let v: Vec<&str> = buf.split('\n').collect();
    let mut cur: u32;
    let mut prev: u32 = 0;
    let mut counter: u32 = 0;
    
    println!("------");

    // iterate over the length of the vector
    // Vec.len() is of type usize, so we cast it here
    for i in 0..(v.len() as u32) {
        
        // parse the value in the vector at position i
        // and make sure that it's a number
        // vector indices are referenced with a usize type, so cast back
        cur = match v[i as usize].parse() {
            Ok(n) => n,
            Err(error) => {
                println!("Not an integer: {:?}", error);
                0
            }
        };

        println!("prev:{}, cur:{}", prev, cur);

        // if the value is not the first entry in the file and the 
        // previous value is less than the current value, then increment
        // the counter
        if i > 0 && prev < cur {
            counter+=1;
            println!("previous {} is less than cur {}, updating counter {}",
            prev, cur, counter);
        }
        prev = cur;
    }
    
    println!("There were {} increased measurements", counter);
    println!("done!");
    Ok(())
}