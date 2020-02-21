//Suppress warnings about unused vars
#[allow(unused_variables)]

// Suppress warnings about unused assigments
#[allow(unused_assignments)]

fn main() {
    println!("Hello, world!");

    // Mutable signed 8-bit integer
    let mut x: bool = true;
    x = false;
    
    
    // Variables are immutable by default
    let y: i8 = 10; // 2^8 possible numbers [-128 to +127]
    //y = 12; // Will crash!
    println!("Min i8 is {}", std::i8::MIN);
    println!("Max i8 is {}", std::i8::MAX);

    //let z: i8 = y + 120; // greater than +127
    //println!("{}", z); // This will crash at runtime (if in debug mode)
    // Will wrap around if in release mode

    // Unsigned 8-bit integer
    let a: i128 = 10;
    println!("Min i8 is {}", std::i128::MIN);
    println!("Max i8 is {}", std::i128::MAX);
}

// > cargo build
// > cargo run   OR    cargo run --release