
struct Color {
red:u8,
green:u8,
blue:u128
}

fn main(){
    let blue = Color{red:0,green:200,blue:400};
    print_color(&blue);
}

fn print_color(c: &Color){
    println!("Color - R:{},G:{},B:{}",c.red,c.green,c.blue)
}



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