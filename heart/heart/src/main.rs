fn main() {
    // Ownership and rules

    // 1. Each value has an owner
    // No value in memory doesn't have a variable who owns it

    // 2. There is only one owner
    // While other variables may borrow values, only one variable owns the value

    // 3. When the owner goes out of scope, the value is immediately dropped
    let s1 = String::from("abc");
    let s2 = s1;
    
    println!("{}", s1) // Raises an error


    // Stack and Heap memory

    // Stack: In order, fixed size values (LIFO)

    // Heap: Unordered variable size values (Tends to be slow)

    // We create s1. A pointer, length and capacity for s1 are pushed to the stack
    // ptr -> Heap (a,b,c) - some bytes in memory
    // len 3 -> length of 3
    // capacity -> 3 
    // All constant time by the way
    let s1 = String::from("abc");

    // a pointer to the byte representing the p1 value is called
    // now the length and capacity of s1 have been transferred to s2
    // 
    let s2 = s1;

    // in memory the characteristics of s1 don't exist until s1 is called again
    // in this way we prevent a 'memory leak' where the application uses twice as much memory
    // as it needs to in order to store a useful and not useful variable
    // with a large system with lots of moving parts a bad memory leak could be fatal

    // s1 is 'uninitialised' right now. the value has 'moved' to s1.
    // if s1 were mutable we could assign it a new value and use it again
    // but it's immutable so it's garbage and un-useable

    let s1 = String::from("abc");
    let s2 = s2.clone();
    println!("{}", s1);



}
