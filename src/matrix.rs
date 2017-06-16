use std::fmt;
// #[derive(Debug)]
pub struct Matrix<T> {
    pub rows: usize,
    pub cols: usize,
    pub data: Vec<T>
}
//
// impl<T> fmt::Debug for Matrix<T> {
//     fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
//         write!(f, "{:?}", self)
//     }
// }

impl<T: fmt::Debug> Matrix<T> {
    pub fn get(&self, x:usize, y:usize) -> &T {
        return &self.data[x+y*self.cols];
    }

    pub fn set(&mut self, x:usize, y:usize, val:T) {
        self.data[x+y*self.cols] = val;
    }

    pub fn get_column(&self, x:usize) -> Vec<&T> {
        let mut column:Vec<&T> = vec![];
        for i in 0..self.rows {
            column.push(self.get(x,i));
        }
        return column;
    }

    pub fn get_row(&self, y:usize) -> Vec<&T> {
        let mut row:Vec<&T> = vec![];
        for i in 0..self.rows {
            row.push(self.get(i,y));
        }
        return row;
    }

    pub fn print(&self) {
        for x in 0..self.cols {
            for y in 0..self.rows {
                println!("x: {:?}, y: {:?}, data: {:?} ", y,x,  self.get(y,x) );
            }
        }
    }
}
