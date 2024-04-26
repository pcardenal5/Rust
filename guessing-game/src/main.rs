use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    let secret_number = rand::thread_rng().gen_range(1..=100);
    
    println!("¡Adivina el número!");
    loop {
        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("No se ha podido leer la línea");
    
        let guess :u32 = match guess.trim().parse(){
            Ok(num) => num,
            Err(_) => {
                println!("Por favor, introduzca un número");
                continue;
            },
        };
        
        match guess.cmp(&secret_number){
            Ordering::Less => println!("¡Demasiado pequeño!"),
            Ordering::Greater=> println!("¡Demasiado grande!"),
            Ordering::Equal=> {
                println!("¡Exacto!");
                break;
            }
        }
    
    }

}