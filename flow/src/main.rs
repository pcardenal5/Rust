/*
Tasks:
    - Convert temperatures between Fahrenheit and Celsius.
    - Generate the nth Fibonacci number.
    - Print the lyrics to the Christmas carol “The Twelve Days of Christmas,” taking advantage of the repetition in the song.
*/
#![allow(non_snake_case)]
use std::collections::HashMap;
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
    // println!("Introduzca el número de fibonacci que desea obtener:");
    let mut previous = HashMap::new();
    for i in 1..185 {
        let fiboN : u128 = i; 

        let fiboTuple: (u128, HashMap<u128,u128>)  = fibo(fiboN, previous.clone());
        previous = fiboTuple.1;
        let fiboNum = previous.get(&fiboN).expect("Error al obtener la variable");
        println!("El número de fibonacci en la posición {fiboN} es {fiboNum}");
    }
    
    
}

fn fibo(n: u128, previous: HashMap<u128,u128> ) -> (u128, HashMap<u128, u128>){
    if n <= 2{
        // Set first two numbers
        let mut initialMap: HashMap<u128, u128> = HashMap::new();
        initialMap.insert(1, 1);
        initialMap.insert(2, 1);
        return (1, initialMap);
    } else {
        if previous.contains_key(&n){
            return (n, previous);
        } else {
            let fN2 = fibo(n-2, previous);
            let fN1 = fibo(n-1, fN2.1);
            let fN: u128 = fN1.1.get(&(n-1)).expect("No existe el elemento") + fN1.1.get(&(n-2)).expect("No existe el elemento");
            let mut previous = fN1.1;
            previous.insert(n, fN);
            return (n, previous);
        }
    }
}