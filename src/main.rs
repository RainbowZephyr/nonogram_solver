use std::io;
use std::error::Error;
use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;
use std::path::Path;

mod matrix;
use matrix::Matrix;

extern crate rand;
fn read_from_file(){
    let mat = Matrix{rows:10, cols:10, data: vec![false;100]};

    println!("{:?}", mat.get(0,0));
    // let path = Path::new("puzzle.txt");
    // let display = path.display();

    // Open the path in read-only mode, returns `io::Result<File>`
    // let file = match File::open(&path) {
    //     // The `description` method of `io::Error` returns a string that describes the error
    //     Err(why) => panic!("couldn't open {}: {}", display, Error::description(&why)),
    //     Ok(file) => file,
    // };
    //
    // // Collect all lines into a vector
    // let reader = BufReader::new(file);
    // let lines: Vec<_> = reader.lines().collect();
    //
    // for l in lines {
    //     for c in l.unwrap().chars() {
    //         if c.to_string() == "[" {
    //             stack.push("[");
    //         } else if c.to_string() == "]" {
    //             stack.pop();
    //         }
    //
    //         println!("{:?}",c);
    //     }
    //
    //     //        if (l.unwrap().contains("a")) {
    //     //        }
    // }
    //
    // println!("{:?}", stack);

}

fn find_longest(arr:&Vec<Vec<u32>>, dimensions:u32 ) {
    let mut sum:u32 = 0;
    let mut index = -1;
    let mut results:Vec<u32> = vec![];

    for row in arr {
        results.push(row.iter().sum());
    }
    // index = results.iter().enumerate().max_by(|&(_, item)| item);
    println!("{:?}", results);
}

// fn fill_matrix(top:&Vec<Vec<u32>>, left:&Vec<Vec<u32>>, matrix:&Matrix) {
//
// }

fn check_errors(top:&Vec<Vec<u32>>, left:&Vec<Vec<u32>>, matrix:&Matrix<bool>){
    let mut x = 0;
    let mut y = 0;
    let pixels:Vec<&bool>;
    for column in top {
        println!("{:?}", matrix.get_column(x));
        for pixel in column {

        }
        x +=1;
    }
}

fn main() {
    //    println!("Enter Line");
    // let mut guess = String::new();
    //    io::stdin().read_line(&mut guess).expect("Failed to read line");
    // let res: Vec<String> = guess.split(" ").map(|s| s.trim().to_string()).collect();
    // println!("{:?}",res);
    let mut stack: Vec<&str> = vec![];
    let top = vec![vec![1,2,1], vec![8], vec![2,2],  vec![1,4,1], vec![2,4,2], vec![2,4,2], vec![1,4,1], vec![2,2], vec![8], vec![1,2,1]];

    find_longest(&top,10);
    // let left = [[1,2,1], [8,0,0], [2,2,0],  [1,4,1], [2,4,2], [2,4,2], [1,4,1], [2,2,0], [8,0,0] , [1,2,1]];

    let mut data:Vec<u8> = vec![];
    for i in 0..36{
        data.push(rand::random::<u8>());
    }

    let mat = Matrix{rows:6, cols:6, data: data};
    mat.print();
    println!("Column: {:?}, data: {:?}", 1, mat.get_column(1));
}
