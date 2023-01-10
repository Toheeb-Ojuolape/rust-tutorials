use std::fs::File;
use std::io::Write;
// use std::io::prelude::*;

fn main(){
let mut file = File::create("output.txt").expect("File could not be created,sorry");
file.write_all(b"I think I like Rust!").expect("Cannot write to a file, sorry");
}


//how to access commandline arguments in rust
// use std::env;

// fn main (){
//     let args: Vec<String> = env::args().collect();

//     for argument in args.iter(){
//         println!("{}",argument);
//     }
// }



//how to read files in rust
// use std::fs::File;
// use std::io::prelude::*;

// fn main(){
//     let mut file = File::open("info.txt").expect("Can't open file!");

//     let mut content = String::new();
//     file.read_to_string(&mut content).expect("Oops!, Cannot read file");

//     println!("File content:\n\n {}",content)

// }
//vector in rust are arrays
// fn main(){
//     let mut my_vector = vec![1,2,3,4];
//     my_vector.push(49);
//     for i in my_vector.iter(){
//         println!("{}",i);
//     }
// }


//traits
// struct Person {
// name:String,
// age:u8
// }

// impl Person {
//     fn to_string(&self) -> String {
//         return format!("My name is {} and my age is {}",&self.name,&self.age)
//     }
// }
// fn main(){
// let dom = Person {name : String::from("Toheeb"),age:24};
// println!("{}",dom.to_string())
// }


//strings
// fn main(){
//     let my_string = String::from("How are you doing");

//     for word in my_string.split_whitespace(){
//         println!("{}",word);
//     }
// }

//Impl keyword is like classes in javascript
// struct Rectangle {
//     width: u32,
//     height: u32
// }

// impl Rectangle{
//     fn print_description(&self){
//         println!("Rectangle: {} x {} = {}", &self.width,&self.height,&self.width*&self.height);
//     }

//     fn is_square(&self) -> bool{ 
//         return &self.width == &self.height;
//     }
// }

// fn main(){
//     let my_rect = Rectangle { width:10, height:10};
//     println!("Is this shape a square? {}", my_rect.is_square());
//     my_rect.print_description();
// }


// fn main(){
//     let array = [2;400];

// for n in array.iter(){
//         println!("{}",n);
//     }

    // let number: [i32;5]= [1,2,3,4,5];
    // for n in number.iter(){
    //     println!("{}",n);
    // }
// }


// struct Color {
// red:u8,
// green:u8,
// blue:u128
// }

// fn main(){
//     let blue = Color{red:0,green:200,blue:400};
//     print_color(&blue);
// }

// fn print_color(c: &Color){
//     println!("Color - R:{},G:{},B:{}",c.red,c.green,c.blue)
// }



//Tuple Struct
// struct Color(u8,u8,u8);

// fn main(){
//     let mut color = Color(244,54,45);

//     color.1 = 30;

//     println!("{}, {}, {}",color.1,color.2,color.0);
// }

// struct Color{
//     red:u8,
//     green:u8,
//     yellow:u8

// }
// fn main(){
//     let mut bg = Color {red: 255, green:70, yellow:15};
//     bg.yellow = 45;
//     println!("{},{},{}",bg.red,bg.green,bg.yellow);
// }

// fn main(){
//     let mut x = 19;
//     let dom  = &mut x;
//     *dom +=1;
//     println!("x is {}", dom);

// }


//Code block and scoping
// fn main(){
//     let x = 10;
//     {
//         let y = 5;

//     }
//     print!("x:{} and y:{}",x,y);
// }


// fn main(){
//     print_function_to(10)
// }

// fn print_function_to(num:u32){
//     for n in 1..num{
//         if is_even(n){
//             println!("{} is even",n);
//         } else {
//             println!("{} is odd",n);
//         }
//     }
// }

// fn is_even(num:u32) -> bool{
//     return num % 2 == 0;
// }



// fn main(){
//     let tup1 = (20,"Rust is cool",30);
//     let (a,b,c) = tup1;
//     println!("a is equal to {}",a);
//     println!("b is equal to {}",b);
//     println!("c is equal to {}",c);

// }


//constants should be in capital letter
// const MAXIMUM_NUMBER:u8 = 20;
// fn main(){
//     for i in 1..MAXIMUM_NUMBER{
//         println!("the number is {}",i);
//     }
// }


//Enums: enums are used to group different object types
// enum Direction {
//     Up,
//     Down,
//     Left,
//     Right
// }

// fn main(){
//     let player_direction:Direction = Direction::Up;

//     //match is like switch statement in javascript
//     match player_direction {
//         Direction::Up => print!("We're heading up"),
//         Direction::Down => print!("We're going all the way down"),
//         Direction::Left => print!("We're going all the way to the left"),
//         Direction::Right => print!("We're going all the way to the right"),

//     }
// }


//for loop in rust
// fn main(){
//     let food = vec!["Beans","potatoes","Rice"];

//     for (index,a) in food.iter().enumerate(){
//         println!("The index {} is for food {}",index,a);
//     }
// }


//while loop in rust
// fn main() {
//     let mut n = 1;
//     while n <= 50{
//         if n % 5 == 0 {
//             println!("n is equal to {}",n);
//         }
//         n += 1;
//     }
// }





// fn main() {
//     let x = 5;

//     let x = x + 1;

//     {
//         let x = x * 2;
//         println!("The value of x in the inner scope is: {x}");
//     }

//     println!("The value of x is: {x}");
// }

// fn main(){
//     let mut n = 0;

//     loop{
//         n+=1;

//         if n == 7{
//             continue;
//         }

//         if n>10{
//             break;
//         }

//         println!("The value of n is {}",n)
//     }
// }