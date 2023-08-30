use std::ops::Add;

fn main() {
    let i: u8 = 1u8;
    let p: &u8 = &i;

    println!("i:\t{}", i);
    println!("p:\t{:p}", p);
    println!("*p:\t{}", *p);

    print!("\n");

    let x: u8 = 1u8;
    println!("x:\t{}", x);
    println!("ptr:\t{:p}", &x);

    print!("\n");
    
    let y: u16 = 1u16;
    let ptr = &y;
    let pi: &u16 = &ptr.add(1);
    println!("pi:\t{:p}", pi);
}
