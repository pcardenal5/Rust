/*
Tasks:
    - Convert temperatures between Fahrenheit and Celsius.
    - Generate the nth Fibonacci number.
    - Print the lyrics to the Christmas carol “The Twelve Days of Christmas,” taking advantage of the repetition in the song.
*/
#![allow(non_snake_case)]
use std::io;

fn main() {
    
    println!("Introduzca una temperatura en Farenheit:");

    let mut fDegrees = String::new();

    io::stdin()
        .read_line(&mut fDegrees)
        .expect("No se ha podido leer la línea");
    
    let fDegrees: f64 =  fDegrees.trim().parse().expect("Por favor, introduzca un número");

    let fCelsius: f64  = fahrenheitToCelsius(fDegrees);
    println!("Ha introducido {fDegrees}ºF, que es equivalente a {fCelsius}ºC");
}

fn fahrenheitToCelsius(fDegrees:f64) -> f64{
    (fDegrees -32f64)/1.8f64
}