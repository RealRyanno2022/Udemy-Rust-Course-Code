fn main() {
    
    // Let can destructure right hand data to initialise variables
    let bunnies = 2; 
    let (bunnies,carrots) = (8,50);
    
    // Immutability = safety
    let bunnies2: i32 = 4;

    // Compiler freaks out
    // It's like backend typescript


    // mutable (can be changed)
    let mut bunnies = 32;

    // constant (requires type annotation)
    // must be an immutable value
    const WARP_FACTOR: f64 = 9.9;

    // - can be put out of module scope and made global
    // - O(1) time

    // Scope - Where you are allowed to use your varaible

    let x = 5;
    {
        let y = 99;
        println!("{}, {}", x, y);
    }
    println!("{}, {}", x, y); // Error!


    {
        x = 99;
        println!("{}", x); // Prints "99"
    }
    println!("{}", x); // Prints 5;

    // x is redefined.
    // Immutable -> mutable
    // The compiler treats it as mutable
    let mut x = 5;
    let x = x;

    // Starts as a string and becomes an image
    let meme = "gk&a"
    let meme = make_image(meme);


    // Memory safety
    // The compiler will let you do something if you can guarantee that it is safe

    let enigma: i32;

    // enigma is guaranteed to be used before it compiles
    if true {
        enigma = 42;
    } else {
        enigma = 7;
    }
    println!("enigma is {}", enigma);

    // C CODE 

    // #include <stdio.h>
    // int main() {
    // int enigma 
    // printf("%d\n", enigma);
    //
    // }

    // C is quite lax about memory safety so this will print '1' for whatever reason

    // Back to Rust.

    
}
