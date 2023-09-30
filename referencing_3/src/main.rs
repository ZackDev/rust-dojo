fn main() {
    // trying to pass the reference of a value to a function
    let mut i: u32 = 4;     // a mutable u32 variable
    println!("{i}");

    add(&mut i, 1); // a mutable 'borrow' of i
    println!("{i}");

    add(&mut i, 1); // another mutalble 'borrow'
    println!("{i}");
}

fn add(r: &mut u32, to_add: u32) {  // r = mutable u32 'borrow', to_add = variable u32
    *r += to_add;
}
