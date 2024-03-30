
// Derive the `fmt::Debug` implementation for `Structure`, a structure which contains a single `i32`.
#[derive(Debug)] // Quitar esta línea hace que la estructura Deep no pueda ser impresa por pantalla...
struct Structure(i32);

// Put a `Structure` inside of the structure `Deep` and make it printable
#[derive(Debug)]
struct Deep(Structure);


// This is the main function.
fn main() {

    // Statements here are executed when the compiled binary is called.

    // Print text to the console.
    println!("Hello World!");
    println!("I'm a Rustacean!");
    

    let x = 5 + 90 +  5;
    println!("Is `x` 10 or 100? x_1 = {} and x_2 = {}", x, x - 45);
    println!("You can also use positional or named  arguments! x_2 = {1} and x_1 = {0} {subject} {verb} {object}", 
        x,
        x - 45, 
        object="the lazy dog",
        subject="the quick brown fox",
        verb="jumps over"
    );

    // Different formatting can be invoked by specifying the format character
    // after a `:`.
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
    println!("{number:0<5}", number=1); // 10000

    // You can use named arguments in the format specifier by appending a `$`.
    println!("{number:0>width$}", number=1, width=5);


    // Rust even checks to make sure the correct number of arguments are used.
    println!("My name is {0}, {1} {0}", "Bond", "James");
    // FIXME ^ Add the missing argument: "James"


    #[allow(dead_code)] // disable `dead_code` which warn against unused module
    // struct Structure(i32);

    // This will not compile because `Structure` does not implement
    // fmt::Display.
    // println!("This struct `{}` won't print...", Structure(3));

    // For Rust 1.58 and above, you can directly capture the argument from a
    // surrounding variable. Just like the above, this will output
    // "    1", 4 white spaces and a "1".
    let number: f64 = 1.0;
    let width: usize = 5;
    println!("{number:>width$}");

    /*
        Activity: Add a println! macro call that prints: 
            Pi is roughly 3.142 
        by controlling the number of decimal places shown. For the purposes of this exercise,
        use let pi = 3.141592 as an estimate for pi. 
        (Hint: you may need to check the std::fmt documentation for setting the number of decimals to display)
    */
    let pi: f64 = 3.141592;

    println!("Pi is roughly {0:.3}", pi);

    /*
    #################
    ##### Debug #####
    #################
    */

    // Printing with `{:?}` is similar to with `{}`.
    println!("{:?} months in a year.", 12);
    println!("{1:?} {0:?} is the {actor:?} name.",
             "Slater",
             "Christian",
             actor="actor's");

    // `Structure` is printable!
    println!("Now {:?} will print!", Structure(3));

    // The problem with `derive` is there is no control over how
    // the results look. What if I want this to just show a `7`?
    println!("Now {:#?} will print!", Deep(Structure(7)));

}