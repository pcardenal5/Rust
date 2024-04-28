/*
Tasks:
    - Convert temperatures between Fahrenheit and Celsius.
    - Generate the nth Fibonacci number.
    - Print the lyrics to the Christmas carol “The Twelve Days of Christmas,” taking advantage of the repetition in the song.
*/
#![allow(non_snake_case)]
use std::io;

fn main() {
    
    println!("Escoja la aplicación:");
    println!("\t1: Coversor de Farenheit a Celsius");
    println!("\t2: Obtener el n-ésimo número de Fibonacci");

    let mut choice = String::new();

    io::stdin()
        .read_line(&mut choice)
        .expect("No se ha podido leer la línea");

    let choice = choice.trim();

    if choice == "1"{
        farenheitHandler();
    } else if choice == "2"{
        fibonacciHandler();
    }
}

fn farenheitHandler(){

    println!("Introduzca una temperatura en Farenheit:");

    let mut fDegrees = String::new();

    io::stdin()
        .read_line(&mut fDegrees)
        .expect("No se ha podido leer la línea");
    
    let fDegrees: f64 =  fDegrees.trim().parse().expect("Por favor, introduzca un número");

    let fCelsius: f64  = farenheitToCelsius(fDegrees);
    println!("Ha introducido {fDegrees}ºF, que es equivalente a {fCelsius}ºC");
}

fn farenheitToCelsius(fDegrees:f64) -> f64{
    (fDegrees -32f64)/1.8f64
}


fn fibonacciHandler(){
    println!("Introduzca el número de fibonacci que desea obtener:");

    let mut fiboN = String::new();

    io::stdin()
        .read_line(&mut fiboN)
        .expect("No se ha podido leer la línea");
    
    let fiboN: u64 =  fiboN.trim().parse().expect("Por favor, introduzca un número");

    let fiboNum: u64  = fibo(fiboN);
    println!("El número de fibonacci en la posición {fiboN} es {fiboNum}");

}

// Terrible implementation, but it's not the point of the program
fn fibo(n:u64) -> u64{
    if n > 2{
        fibo(n-1) + fibo(n-2)
    } else{
        1
    }
}