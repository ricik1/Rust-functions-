//import our random number generator
use rand::Rng;

//build our dice
//define a struct to hold a number of sides to our dice
//strust holds data not functions
// 8 sided dice
struct Dice {
sides: u8,


}
// crreat a fn to print title



// Create a funtion to roll the dice= implement a method from our dice struct for out dice
impl Dice {
    fn roll(&self) -> u8 {
//generate a random numbner between q and the numbner of sided
let mut rng = rand ::thread_rng();
rng.gen_range(1.. self.sides +1)

}
}
//create our main fn is entry point for program
fn main(){
   // creat a new dice with 6 sided
let d6 = Dice {sides: 06};


// creat a new dice with 8 sided
let d8 = Dice {sides: 08};

// creat a new dice with 12 sided
let d12 = Dice {sides: 12};

// creat a new dice with 20 sided
let d20 = Dice {sides: 20};


// roll all dice first**********************
//roll the dice
let d6_roll = d6.roll();

//roll the dice
let d8_roll = d8.roll();

//roll the dice
let d12_roll = d12.roll();

//roll the dice
let d20_roll = d20.roll();

//Print title
println!(" \n\n\t*** Dice Roller***");

//THEN print the result for all my dice**************************
println!("with our 6 sided dice- You rolled a {}", d6_roll);
println!("with our 8 sided dice- You rolled a {}", d8_roll);
println!("with our 12 sided dice- You rolled a {}", d12_roll);
println!("with our 20 sided dice- You rolled a {}", d20_roll);
println!(" \n\n\t*** Thanks for playing***");

//println!("with our 6 sided dice- You rolled a {}", d6.roll()); this is a short cut 
// to roll then print in one line.
// kep all the code the same to look better

}