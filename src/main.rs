use std::io;
use std::error::Error;
use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;
use std::path::Path;

mod matrix;

use matrix::Matrix;

extern crate rand;

fn read_from_file() {
    let mat = Matrix { rows: 10, cols: 10, data: vec![false; 100] };

    println!("{:?}", mat.get(0, 0));
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

fn find_longest(arr: &Vec<Vec<u32>>, dimensions: u32) {
    let mut sum: u32 = 0;
    let mut index = -1;
    let mut results: Vec<u32> = vec![];

    for row in arr {
        results.push(row.iter().sum());
    }
    // index = results.iter().enumerate().max_by(|&(_, item)| item);
    println!("{:?}", results);
}

// fn fill_matrix(top:&Vec<Vec<u32>>, left:&Vec<Vec<u32>>, matrix:&Matrix) {
//
// }

fn check_top_errors(top: &Vec<Vec<u32>>, matrix: &Matrix<bool>) -> isize {
    let mut x = 0;

    //Column Check
    let mut pixels: Vec<&bool>;
    let mut count: Vec<u32>;
    for column in top {
        pixels = matrix.get_column(x);
        count = count_pixels(&pixels);

        if !count.iter().zip(column.iter()).all(|(cou, col)| cou == col) {
            return x as isize;
        }

        x += 1;
    }

    return -1;
}

fn check_left_errors(left: &Vec<Vec<u32>>, matrix: &Matrix<bool>) -> isize {
    let mut y = 0;

    //Row Check
    let mut pixels: Vec<&bool>;
    let mut count: Vec<u32>;
    for row in left {
        pixels = matrix.get_row(y);
        count = count_pixels(&pixels);

        if !count.iter().zip(row.iter()).all(|(c, r)| c == r) {
            return y as isize;
        }

        y += 1;
    }


    return -1;
}


fn count_pixels(pixels: &Vec<&bool>) -> Vec<u32> {
    let mut counter: u32 = 0;
    let mut count: Vec<u32> = vec![];
    for i in 0..pixels.len() {
        if i != 0 && *pixels[i] == false {
            count.push(counter);
            counter = 0;
        } else if *pixels[i] == true {
            counter += 1;
        }
    }
    return count;
}

fn main() {
    //    println!("Enter Line");
    // let mut guess = String::new();
    //    io::stdin().read_line(&mut guess).expect("Failed to read line");
    // let res: Vec<String> = guess.split(" ").map(|s| s.trim().to_string()).collect();
    // println!("{:?}",res);
    let mut stack: Vec<&str> = vec![];
    let top = vec![vec![1, 2, 1], vec![8], vec![2, 2], vec![1, 4, 1], vec![2, 4, 2], vec![2, 4, 2], vec![1, 4, 1], vec![2, 2], vec![8], vec![1, 2, 1]];

    //    find_longest(&top, 10);
    // let left = [[1,2,1], [8,0,0], [2,2,0],  [1,4,1], [2,4,2], [2,4,2], [1,4,1], [2,2,0], [8,0,0] , [1,2,1]];

    let mut data: Vec<u8> = vec![];
    let mut bools: Vec<bool> = vec![];
    for i in 0..100 {
        //        data.push(rand::random::<u8>());
        bools.push(false);
    }

    //    assert_eq!(top[2], top[1]);

    let mat = Matrix { rows: 6, cols: 6, data: data };
    let img = Matrix { rows: 10, cols: 10, data: bools };
    println!("{:?}", check_top_errors(&top, &img));
    //    mat.print();
    //    println!("Column: {:?}, data: {:?}", 1, mat.get_column(1));
}
