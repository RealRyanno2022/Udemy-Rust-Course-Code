fn main() {
   // Section 5: The Meat of Rust
   //
   // Structs
   //

    struct RedFox {
        enemy: bool,
        life: u8,
    }
    
    // you must initialise objects for each struct
    
    let fox = RedFox {
        enemy: true,
        life: 70,
    };

    // we can use an associated function to define defaults
    
    impl RedFox {
        fn new() -> Self {
            Self {
                enemy: true,
                life: 70,
            }
        }
    }
    
    let fox = RedFox::new();

    // scope (::) allows you to access the function's namespace
    // this refers to the new function
    

    impl RedFox {
        fn function() ...


        fn move(self) ...
        
        fn borrow(&self) ...

        fn mut_borrow(&mut self) ...
    }
    
    // Fuck your object-orientated programming, there is no struct inheritance. 
    //


    // Traits
    //
    //
    // Instead we have traits
    //

    struct RedFox {
        enemy: bool,
        life: u32,
    }

    trait Noisy {
        fn get_noise(&self) -> &str;
    }


    // These are OK implementations but we can use traits instead

    impl Noisy for RedFox {
        fn get_Noise(&self) -> &str { "Meow?" }
    }

    impl RedFox {
        fn get_noise(&self) -> &str { "Meow?" }
    }

    // See here
    //

    fn print_noise<T: Noisy>(item: T) {
        println!("{}|", item.get_noise());
    }

    // We can get noisy to work for other type values too. This is a decent way to get around
    // having to use strings.
    //

    impl Noisy for u8 { // a byte
        fn get_noise(&self) -> &str { "BYTE!" }
    }

    fn main() {
        print_noise(5_u8); // prints "BYTE!"
    }

    // Copy trait
    // Implemented by simple primitive types 
    // Type with copy traits are copied instead of moved in move situation
    // Good for small values that fit entirely on the stack
    // types which use the heap cannot use copy

    // Traits can inherit from other traits
    //
    // Don't provide a second implementation of a trait for a new object

    trait Run {
        fn run(&self) {
            println!("Running");
        }
    }
    
    struct Robot {}
    impl Run for Robot {}
    

    fn main() {
        let robot = Robot {}
        robot.run():
    }

    // The struct code will run
    // You can't define fields as part of traits
    //


    // Collections
    //

    Vec<T> // LinkedList / Array

    let mut v: Vec<i32> = vec::new();
    v.push(2);
    v.push(4);
    v.push(6);

    let x = v.pop(); // x is 6
    println!("{}", v[1]); // print 4
    
    let mut v = vec![2,4,6]; // vec! is a macro
                             // you can do a lot of the same stuff as in java


    HashMap<K,V> // O(1) HashMap

        let mut h: HashMap<u8, bool> = HashMap::new();
        h.insert(5,false);
        h.insert(6,true);
        let have_five = h.remove(&5).unwrap(); // Checks if the hashmap has five
                                               //
    

    // Other data structures


    VecDeque // Double ended queue, efficient at add/removing front/back items, but everything else
             // is a little less efficient than a vector

    HashSet // Hash implementation of a set that performs set operations very efficiently
 
    LinkedList // Quick at adding/removing arbitrary items but slow everywhere else.
    
    BinaryHeap // Priority queue which always pops off the max value
    
    BTreeSet   // Used over hash variants when you need the map keys/set values to always be in
               // order
    BTreeMap


    // Enums
    

    // Like Haskell algrebraic properties

    enum color {
        Red, 
        Green,
        Blue,
    }

    let color = Color::Red;

    // Rust allows you to associate the data and the methods with their variants
    //
    
    enum DispenserItem {
        Empty,
        Ammo(u8),
        Things(String, i32),
        Place {x: i32, y: i32},
    }

    use Dispenseritem::*;
    let item = ? // Could be anything!
    


    // Highly useful

    enum Option<T> {
        Some(T);
        None,
    }


    // "if let" takes a pattern that will match one of the variants

    if let Some(x) = my_variable {
        println!("value is {}", x);
    }


    // If you want to consider all patterns, use match

    // You must consider all possible outcomes here!

    match my_variable {
        Some(x) => {
            println!("value is {}", x);
        },
        None => {
            println!("no value");
        }.
    }


    // Option is always in scope in std library by default
    //


    // Result (enum)
    //
    // Used a lot to prevent I/O errors

    #[must_use]

    enum Result<T,E> {
        Ok(T),
        Err(E),
    }

    // This retunrs a result because the file may not open successfully


    use std::fs::File;

    fn main() {
        File::open("foo");
    }

    // note: #[warn(unused_must_use)] on by default
    // note: this 'Result' may be an 'Err' variant, which should be handled


    use std::fs::File;
    
    fn main() {
        let res = File::open("foo");
        let f = res.expect("error message");
    }


    fn main() {
        let res = File::open("foo");
        if res.is_ok() {
            let f = res.unwrap();
        }
    }

    // Need to come back to exercise E,F,G,H

    // Section 6: Final Lectures

    // Closures
    //

    // Anonymous function that can borrow or capture some data from the scope it is nested in
    
    let add = |x, y| { x + y }

    // You can call this later
    // Types/args are inferred based on use
    //


    add(1, 2); // 3
    

    // Or just use no parameters

    || { x + y }
    
    // Example
    // s lives while the closure is in scope. 
    // We can send the closure to another thread or return it as the value of a function

    let s = "S".to_string();
    
    let f = || {
        println!("{}",s);
    };

    f();

    // A lot of it is down to functional preference
    // See here

    let mut v = vec![2,4,6];

    v.iter()
        .map(|x| x * 3)
        .filter(|x| *x > 10)
        .fold(0, |acc, x| acc + x);


    // Threads

    // Portable - works on Mac/Linux/Windows etc.


    use std::thread;

    fn main() {
        let handle = thread::spawn(move || { // closure with no arguments as main
            // Do stuff in a child thread
        });

        // Do stuff simultaneously in the main htread
        
        // Wait until thread has exited

        handle.join().unwrap();

    






































































    


















}
