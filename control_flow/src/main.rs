fn main() {
    // Scalars

    // Integers, floats, booleans, characters

    // INTEGERS
    // Unsigned: u(n * 8) up to 128 bits
    // usize is the size of the platform's pointer type
    // usize can represent every memory address in the process
    // Signed is the same just use i instead of u
    // u is used to find array index
    // the upper bound of object and array size is the maximum isize value

    // i32 is the default (Fastest integer, even on 64bit)

    // Integer literals - 
    // Decimal (10000000)
    // Hex - 0xdeadbeef
    // Octal - 0o77543211
    // Binary - 0b11110011
    // Byte (u8 only) - b'A'

    let x = 5u16; // here is a u16 integer
    let y = 3.14f32; // here is an f32 float

    // Strings and source files are UTF-8 which allows you to use funky emojis and symbols




    // Compound Types

    // Gather up to 12 values of other types into one type

    // Tuple
    let info: (u8, f64, i32) = (1,3.3,999);

    // Members of tuples may not always be the same type
    let jets = info.0;
    let jets = info.1;

    // Destructuring
    let (jets, fuel, ammo) = info;

    // Array
    // Same type, lose functionality at size >32!
    // We like to use vec instead of arrays in Rust

    let buf: [u8: 3] = [1,2,3];


    // Control Flow


    // Rust doesn't like type coercion


    if num == 5 {                       msg = if num == 5 {
        msg = 'five';                       "five"
    } else if num == 4 {        ---->   } else if num == 4 {
        msg = "four";                       "four"
    } else {                            } else {
        msg = "other";                      "other"
    }                                   }; // 1. no ;'s: tail expressions
                                           // 2. return would return out of functoin
                                           // 3. All blocks return same type
                                           // 4. Semicolon at end of if statement
   
    // Or to be funky,
    msg = if num == 5 { "five" } else if num == 4 { "four" } else { "other" };

    // We avoid ternary operators
    num = if a { b } else { c };

    // And to nest,
    num = if a {
        if x { y } else { y }
    } else {
        c
    };

    // Unconditional loop

    loop {
        break;
    }

    // Annotate the loop you want to break out of with a label

    'bob: loop {
        loop {
            loop {
                break 'bob;
            }
        }
    }
    
    'bob: loop {
        loop {
            continue; // breaks the outermost loop
        }
    }

    // While loop will only stop when the condition is strictly false

    while (boolean) {
        party();
    }

    // There's no dowhile loop in Rust and the above is just a pretty way to write

    loop {
        if !boolean() { break }
        party();
    }

    // Swap if and party() to effectively get a dowhile loop



    // The for loop

    for num in [7,8,9].iter() {
        // Do stuff with num
    }

    let array = [(1,2), (3,4)];

    for (x,y) in array.iter() {
        // Do stuff with x and y
    }

    // Ranges
    // This is 0 to 49
    for num in 0..50 {
        // Do something with num
    }

    // This is 0 to 50
    for num in 0..=50 {
        // Do something with num
    }



    // Strings
    // Difficult
    
    // str <- string slice, unmodifiable
    // &str <- string
    // String <- modifiable
    

    // &str: pointer to some bytes and a length
    // String: pointer to some bytes, a length and capacity (may be > than what is used)

    // the &str is a subset of the String
    // both are valid UTF-8
    // Because strings are unicode, we can't iterate through strings
    // This is because strings are stored in different bytesizes


    // Indexing operations on standard library operations are constant time while strings aren't

    // you can index simple english text overlapping ASCII in this way
    word.bytes();

    // goes through the scalars
    words.chars();

    // there are a lot of helper functions
    // .nth(p) can be used to index the p'th value of an interator



    // Earmark: finish challenge




































    
}
