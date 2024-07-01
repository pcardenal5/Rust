#![allow(dead_code)]

use rand::Rng;

#[derive(Debug)]
enum Coin{
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
    Dollar
}

#[derive(Debug)]
enum UsState {
    Alaska,
    Texas
}

fn value_in_cents(coin : &Coin) -> u8{
    match coin {
        Coin::Penny => {
            println!("Lucky penny!");
            1
        }
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state ) => {
            println!("The quarter is from {state:?}!");
            25
        },
        Coin::Dollar => 100
    }
}

fn add_1(value : Option<i32>) -> Option<i32>{
    match value {
        None => None,
        Some(number) => Some(number+1)
    }

}

fn roll_dice() -> u8 {
    rand::thread_rng().gen_range(1..8)
}

fn main() {
    let coin : Coin = Coin::Quarter(UsState::Alaska);
    let value = value_in_cents(&coin);
    println!("The value in cents of a {coin:?} is {value}");

    // Add one to value if exists
    let x = Some(5);
    let x1 = add_1(x);
    let x2 = add_1(None);

    println!("Some(5)+1 = {x1:?}, None + 1 = {x2:?}");
    
    // Loop will only break when dice rolls 3 or 7
    loop {
        let dice_roll  = roll_dice();

        match dice_roll {
            3 => {
                println!("You got a cool hat!");
                break
            
            }
            7 => {
                println!("You lost a cool hat...");
                break
            }
            _ => println!("Roll again!")
        }        
    }

}
