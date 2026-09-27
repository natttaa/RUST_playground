mod my_funcs;
mod other_func;
//Дефолтное значение immutable i32

//сначала папка, потом файл, потом функция
use crate::my_funcs::add_five;
use crate::other_func::func_ten::func_ten;

//use crate::my_funcs::{add_five, break_twenty, low_ten}
//use crate::my_funcs::*

fn main() {
    let mut x: u32 = 50;
    println!("x is {}", x);

    let y: u32 = add_five(x);
    println!("y is {}", y);
    
    x = 60;
    println!("x is {}", x);

    let z: u32 = func_ten(y);
    println!("z is {}", z);
}
