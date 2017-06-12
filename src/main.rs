use std::io;
use std::error::Error;
use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;
use std::path::Path;

fn main() {
//    println!("Enter Line");
    let mut guess = String::new();
//    io::stdin().read_line(&mut guess).expect("Failed to read line");
    let res: Vec<String> = guess.split(" ").map(|s| s.trim().to_string()).collect();
    println!("{:?}",res);
    let mut stack: Vec<&str> = vec![];
//    let left = [[1,2,1], [8], [2,2],  [1,4,1], [2,4,2], [2,4,2], [1,4,1], [8] , [1,2,1]];
//    let top  = [[1,2,1], 8, [2,2],  [1,4,1], [2,4,2], [2,4,2], [1,4,1], 8 , [1,2,1]];

    let path = Path::new("puzzle.txt");
    let display = path.display();

    // Open the path in read-only mode, returns `io::Result<File>`
    let file = match File::open(&path) {
        // The `description` method of `io::Error` returns a string that describes the error
        Err(why) => panic!("couldn't open {}: {}", display, Error::description(&why)),
        Ok(file) => file,
    };

    // Collect all lines into a vector
    let reader = BufReader::new(file);
    let lines: Vec<_> = reader.lines().collect();

    for l in lines {
        for c in l.unwrap().chars() {
           if c.to_string() == "[" {
              stack.push("[");
           } else if c.to_string() == "]" {
               stack.pop();
           }

            println!("{:?}",c);
        }

        //        if (l.unwrap().contains("a")) {
//        }
    }

    println!("{:?}", stack);

}
