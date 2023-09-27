#![allow(non_snake_case)]

//single line comment

/*


*/ 


use std:: {i8, i16, i32, u8, u16, u32, u64, isize, usize,
f32, f64};

use std::io::stdin;


fn main() {
    println!("Hello, world!");

    let num: i8 = 10;
    println!("the number is {}", num);

    let name: &str = "Bek";
    let sur_name: &str = "Brace";
    println!("My name is {} {}", name, sur_name )

}
