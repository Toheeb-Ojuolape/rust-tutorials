#![allow(dead_code)]
extern crate serde_json;
extern crate serde;
#[macro_use]
extern crate serde_derive;
// use serde_json::Value as JsonValue;


#[derive(Serialize,Deserialize)]
struct Person{
    name:String,
    age:u8,
    is_male:bool
}



fn main(){
    let json_str = r#"
      {
        "name":"Tobi",
        "age":24,
        "is_male":true
      }
    "#;
    let res = serde_json::from_str(json_str);

    if res.is_ok(){
        let p:Person = res.unwrap();
        println!("User's name is {}",p.name)
    } else{
        println!("Sorry, could not pass json")
    }
}

// struct Rectangle{
//     width:u8,
//     height:u8
// }

// impl Rectangle{
//     fn is_square(&self)->bool{
//         self.width == self.height
//     }
// }

// fn main(){

// }

// fn give_two() ->i32{
//    2
// }


// #[cfg(test)]
// mod code_tests{
//     #[test]
//     #[should_panic]
//     fn test_basic(){
//         assert!(1==1);
//         panic!("Oh no")
//     }
//     #[test]
//     fn test_equals(){
//         assert_eq!(super::give_two(),1+1);
//         assert_ne!(super::give_two(),1+2);
//     }
//     #[test]
//     #[should_panic]
//     fn test_struct(){
//         let r = super::Rectangle{width:50,height:50}; 
//         assert!(r.is_square());
//         panic!("Oh no")
//     }
// }

// use std::process::Command;


// fn main(){
//     let mut cmd = Command::new("python");
//     cmd.arg("tobi.py");

//     //execute the command
//     match cmd.output(){
//         Ok(o) =>{
//             //do stuff here
//             unsafe{
//                 println!("Output: {}",String::from_utf8_unchecked(o.stdout));
//             }
//         }
//         Err(e) =>{
//             println!("There was an error {}",e)
//         }
//     }
// }


// enum Day{
//     Monday, Tuesday, Wednesday,Thursday,Friday,Saturday,Sunday
// }

// impl Day{
//     fn is_weekday(&self)->bool{
//         match self{
//             &Day::Saturday | &Day::Sunday => return false,
//             _ => return true
//         }
//     }
// }

// fn main(){
//   let d = Day::Saturday;
//   println!("Is this day a weekday? {}",d.is_weekday())
// }



//take another tutorial on async rust
// extern crate reqwest;
// use tokio::runtime::Runtime;

// mod apicall{
//     pub async fn get_request(){
//         let response_text =reqwest::get("http://youtube.local/hello")
//         .await.expect("Couldn't make a request")
//         .text().await.expect("Could not read the response text");
//         println!("Response Text:{}",response_text)
//     }
// }

// fn main (){
//     let mut rt = Runtime::new().unwrap();
//     rt.block_on(async {
//         apicall::get_request().await;
//     });
// }

// fn main(){
//    println!("Occupation is: {}", match get_occupation("Tobi"){
//     Some(occupation) => occupation,
//     None => "This user is jobless" 
//    });
// }

// fn get_occupation(name:&str) -> Option<&str>{
//     match name{
//         "Domenic" => Some("Software Engineer"),
//         "Martin" => Some("Dentist"),
//         "Tobi" => Some("Bitcoin developer"),
//         _ => None
//     }
// }



//  mod module {
//  fn manual(){
//     println!(
//         "God abeg"
//     );
//  }
//  pub fn  print_message(){
//     manual();
//     println!("How you dey");
//  }
// }

// fn main(){
//     module::print_message();
// }


// extern crate regex;
// use regex::Regex;

// fn main(){
//     let re = Regex::new(r"(\w{4})").unwrap();
//     let text = "tobi";
//     match re.captures(text){
//         Some(caps)=> println!("Found match {}", &caps[0]),
//         None => println!("Could not find match")
//     }
// }



// mod tobi;

// fn main(){
//  tobi::print_message();
// }



//string methods
// fn main(){
//     {
//         let my_string:String = String::from("Rust is really cool");
//         println!("{}",my_string.replace("cool", "awesome"));
//     }

//     {
//         let my_string = String::from("The weather is \nnice\noutside mate!");
//         for line in my_string.lines(){
//             println!("[ {} ]",line);
//         }
//     }

//     {
//         let my_string = String::from("Leave+a+like+if+you+dare");
//         let tokens:Vec<&str> = my_string.split("+").collect();

//         println!("{}",tokens[2]);
//     }

//     {
//         let my_string = String::from("        My name is Tobi    \n\r");
//         println!("Before trim {}",my_string);
//         println!("After trim {}",my_string.trim())
//     }

//     {
//         let my_string = "decode_youtube";

//         match my_string.chars().nth(4){
//             Some(c) => println!("Character at index 4 is:{}",c),
//             None => println!("No character at index 4"),
        

//     }
// }
// }

// extern crate rand;
// use rand::Rng;

// fn main(){
//     let random_number = rand::thread_rng().gen_range(1..30);
//     println!("Random Number {}",random_number);
// }
//hashmaps
// use std::collections::HashMap;

// fn main(){
//     let mut marks = HashMap::new();

//     marks.insert("Rust Programming",98);
//     marks.insert("Golang programming",75);
//     marks.insert("Typescript programming",98);
//     marks.insert("Flutter programming",75);

//     println!("How many courses I did? {}",marks.len());

//     match marks.get("Flutter programming"){
//         Some(mark)=>println!("You got a score of {} in Golang",mark),
//         None => println!("You didn't offer this course"),

//     }

//     //remove a value
//     marks.remove("Flutter programming");

//     for (subject,mark) in marks{
//         println!("{}, {}",subject,mark)
//     }
// }


//match feature that is like switch
// fn main(){
//     let number= 2;

//     match number {
//         1 => println!("It is the one"),
//         2 => println!("There is two of them"),
//         3 => println!("It doesn't match"),
//         i32::MIN..=0_i32 | 4_i32..=i32::MAX => todo!()
//     }
// }

// struct Person{
//     name:String,
//     age:i32
// }

// trait HasVoiceBox{
//     //can the person speak
//     fn speak(&self);
//     fn can_speak(&self)->bool;
// }

// impl HasVoiceBox for Person {
//     fn speak(&self){
//         println!("Hello, my name is {}",&self.name)
//     }

//     fn can_speak(&self)->bool{
//         if self.age > 2 {
//             return true;
//         }
//         else{
//             return false;
//         }
//     }
// }

// fn main(){
//     let person = Person{
//         name:String::from("Salma"),
//         age:1,
//     };
//     println!("Can {} speak? {} ", person.name,person.can_speak());
// }


//writing a file to rust
// use std::fs::File;
// use std::io::Write;
// // use std::io::prelude::*;

// fn main(){
// let mut file = File::create("output.txt").expect("File could not be created,sorry");
// file.write_all(b"I think I like Rust!").expect("Cannot write to a file, sorry");
// }


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