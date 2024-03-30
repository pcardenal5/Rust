
// This is the main function.
fn main() {

    let x : i16 = 5 + 90 + 5;
    println!("
Como ves, se pueden usar argumentos posicionales o con nombres x_2 = {1} and x_1 = {0} 
{sujeto} {verbo} {predicado}",
        x,
        x - 45,  
        predicado = "whisky",
        sujeto = "Mi pequeño ex-jefe loco",
        verbo = "gozaba vertiendo"
    );
    
    // Se puede dar formato usando caracteres despues de `:`
    println!("Base 10:               {}",   69420); 
    println!("Base 2 (binary):       {:b}", 69420); 
    println!("Base 8 (octal):        {:o}", 69420); 
    println!("Base 16 (hexadecimal): {:x}", 69420); 
    // You can right-justify text with a specified width. This will
    // output "    1". (Four white spaces and a "1", for a total width of 5.)
    println!("{number:>5}", number=1);

    // You can pad numbers with extra zeroes,
    println!("{number:0>5}", number=1); // 00001
    // and left-adjust by flipping the sign. This will output "10000".
    println!("{number:7<5}", number=1); // 10000

    // You can use named arguments in the format specifier by appending a `$`.
    println!("{number:0>width$}", number=1, width=5);

    let pi: f64 = 3.1415926513;
    println!("Pi es aproximadamente {0:.3}", pi);// -> "3.142"
    
    // Printing with `{:?}` is similar to with `{}`.
    println!("{1:?} {0:?} es el nombre del {actor:?}.",
    	"Mota",
    	"José",
    	actor="actor"
    );
}