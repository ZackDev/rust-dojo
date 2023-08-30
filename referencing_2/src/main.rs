use std::mem;

fn main() {
    /*
     * assumptions on referencing
     * 
     * a reference is established with the <&> symbol
     * <i> holds the value, while <p> is a reference to the value of <i>
     * -> thus, changing <i> also changes <p>
     * 
     * --
     * 
     * <&> is not referencing, it is borrowing. borrowing changes ownership.
     */
    let mut i: u8 = 1u8;                // allocates <1u8> on the stack and sets ownership to <i>
    let p: u8 = i;                      // copies and allocates the value of i on the stack and sets ownership to <p>
    
    println!("i: {i} p: {p}");
    i += 1;
    preserve_val(i);
    println!("i: {i} p: {p}");

    /* */
    let x: Box<u8> = Box::new(1u8);     // allocates <1u8> on the heap and sets ownership to <x>
    println!("x: {x}");
    println!("size(x): {}", mem::size_of_val(&x));
    println!("x: {x}");
    destroy_val(x);
}

fn preserve_val(x: u8) {
    println!("this preserves {x}");
}

fn destroy_val(y: Box<u8>) {
    println!("this destroys {y}")
}