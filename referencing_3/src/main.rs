fn main() {
    // trying to pass the reference of a value to a function
    let mut i: u32 = 4;
    println!("{i}");

    add(&mut i, 1);
    println!("{i}");

    add(&mut i, 1);
    println!("{i}");
}

fn add(r: &mut u32, to_add: u32) {
    *r += to_add;
}
