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
    Texas,
    California,
    Montana,
    NewMexico,
    Arizona,
    Nevada,
    Colorado,
    Oregon,
    Wyoming,
    Michigan,
    Minnesota,
    Utah,
    Idaho,
    Kansas,
    Nebraska,
    SouthDakota,
    Washington,
    NorthDakota,
    Florida,
    Oklahoma,
    Missouri,
    Georgia,
    Wisconsin,
    Illinois,
    Iowa,
    NewYork,
    NorthCarolina,
    Virginia,
    Arkansas,
    Alabama,
    Louisiana,
    Mississippi,
    Pennsylvania,
    Ohio,
    Tennessee,
    Kentucky,
    Maine,
    Indiana,
    SouthCarolina,
    WestVirginia,
    Maryland,
    Hawaii,
    Massachusetts,
    Vermont,
    NewHampshire,
    NewJersey,
    Connecticut,
    Delaware,
    RhodeIsland
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

fn main() {
    let coin : Coin = Coin::Quarter(UsState::Alabama);
    let value = value_in_cents(&coin);
    println!("The value in cents of a {coin:?} is {value}")
}
