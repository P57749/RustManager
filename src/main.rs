#![allow(non_snake_case)]

//single line comment

/*


*/ 


use std:: {i8, i16, i32, u8, u16, u32, u64, isize, usize,
f32, f64};

use std::io::stdin;
const _THE_CONST_INT: u8 = 10; 
const _THE_CONST_STR: &str = "cairo"; 


fn main() {
    println!("Hello, world!");

    let num: i8 = 10;
    println!("the number is {}", num);

    let name: &str = "Bek";
    let sur_name: &str = "Brace";
    println!("My name is {} {}", name, sur_name );

    let mut num = 100;

    println!("the number is {}", num);


    //booleans
    let is_valid: bool = true;
    println!("this is a boolean {}", is_valid);

    //characters
    let one_char: char = 'a';

    // floats
    let the_float: f32 = 2.589; 
    println!("this is a float {}", the_float);

    
    println!("My fav char is {}", one_char);
    let(country, capital) = ("Cuba", "La Havana");
    println!("{}, {}", country, capital); 

    //operators for math calculation
    println!("20 + 4 = {}", 20 + 4);
    println!("20 - 4 = {}", 20 - 4);
    println!("20 * 4 = {}", 20 * 4);
    println!("20 / 4 = {}", 20 / 4);
    println!("20 % 4 = {}", 20 % 4);

    //let name: &str = rust;
    println!("The integrer is : {}, and the city is : {}", _THE_CONST_INT, _THE_CONST_STR);
    


    //conditionals
    let _x = 15;

    if _x <= 20 {
        println!("you are less than 20 y.o");
    } else if _x < 30 {
        println!("you are less than 30 y.o");
    }else {
        println!("you have mor than 30 y.o");
    }

    let mut n = 0;

    loop {
        n+=1;// incrrementing by 1
        //n = n + 1;
        println!("the value of n is {}", n);

        if n == 200{
            return break;
        }
    }



    // for i in range 

 

}