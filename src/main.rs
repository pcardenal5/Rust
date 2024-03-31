// Import the `fmt` module to make it available.
use std::fmt;

struct Structure(i32);

impl fmt::Display for Structure {

	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		/*
		Se escribe el primer elemento de la estructura al stream de salida, `f`.
		Devuelve un `fmt::Result` que indicará si la operación ha fallado o no
		*/
		
		write!(f, "{}", self.0)
	}
}

#[derive(Debug)]
struct MinMax(i64, i64);

// Implement `Display` for `MinMax`.
impl fmt::Display for MinMax {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // Use `self.number` to refer to each positional data point.
        write!(f, "({}, {})", self.0, self.1)
    }
}

// Define a structure where the fields are nameable for comparison.
#[derive(Debug)]
struct Point2D {
    x: f64,
    y: f64,
}

// Similarly, implement `Display` for `Point2D`.
impl fmt::Display for Point2D {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // Customize so only `x` and `y` are denoted.
        write!(f, "x: {}, y: {}", self.x, self.y)
    }
}

#[derive(Debug)]
struct Complex {
    real: f64,
    imag: f64,
}

// Similarly, implement `Display` for `Complex`.
impl fmt::Display for Complex {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let sign = if self.imag < 0.0 { "" } else {"+"};
        write!(f, "{0} {2}{1}i", self.real, self.imag, sign)
    }
}


// Define a structure named `List` containing a `Vec`.
struct List(Vec<i32>);

impl fmt::Display for List {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		// Extract the value using tuple indexing, and create a reference to vec
		let vec = &self.0;
		
		write!(f, "[")?;  
		// Iterate over `v` in `vec` while enumerating the iteration
		// count in `count`.
		for (count, v) in vec.iter().enumerate() {
			// For every element except the first, add a comma.
			// Use the ? operator to return on errors.
			if count != 0 {
				write!(f, ", ")?;
			}
			write!(f, "{0}: {1}", count, v)?;
		}
		  
		// Close the opened bracket and return a fmt::Result value.
		write!(f, "]")
	}
}

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

    
    println!("----------------------------------");
    println!("------------- Display ------------");
    println!("----------------------------------");
    
    println!("Compare structures:");
    let minmax = MinMax(0, 14);
    println!("Display: {}", minmax);
    println!("Debug: {:?}", minmax);

    let big_range =   MinMax(-300, 300);
    let small_range = MinMax(-3, 3);

    println!("The big range is {big} and the small is {small}",
             small = small_range,
             big = big_range);

    let point = Point2D { x: 3.3, y: 7.2 };

    println!("Compare points:");
    println!("Display: {}", point);
    println!("Debug: {:?}", point);

    let z = Complex { real: 3.3, imag: 7.2 };

    println!("Compare points:");
    println!("Display: {}", z);
    println!("Debug: {:?}", z);

    let v = List(vec![1, 2, 3]);
    println!("{}", v);

}