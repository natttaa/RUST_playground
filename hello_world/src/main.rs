mod my_funcs;

//Дефолтное значение immutable i32


use crate::my_funcs::add_five;
//use crate::my_funcs::{add_five, break_twenty, low_ten}

fn main() {
    let mut x: u32 = 50;
    println!("x is {}", x);

    let y: u32 = add_five(x);
    println!("y is {}", y);
    
    x = 60;
    println!("x is {}", x);
}
