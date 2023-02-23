fn main() {
    let a: [char; 0] = [];

    // declare array as mutable, to be able to change it's content
    let mut b: [char; 3] = ['x', 'y', 'a'];
    println!("initial array:");
    println!("{:?}", b);
    println!("mutate_locally()");
    mutate_locally(b, 2, 'z');
    println!("{:?}", b);
    println!("mutate()");
    mutate(&mut b, 2, 'z');
    println!("{:?}", b);
    println!("what else?");
    println!("check vectors...");
    let mut v: Vec<u8> = Vec::new();
    println!("initial vector:");
    println!("{:?}", v);
    v.push(16u8);
    println!("added 16:");
    println!("{:?}", v);
    let i = v.pop();        // returns Option type, could return None too.
    let j = v.pop();
    print_some_or_none(i);
    print_some_or_none(j);
    let result_ok = Ok::<u8, i8>(8u8);
    println!("{:?}", result_ok);
    let result_err = Err::<u8, i8>(8i8);
    println!("{:?}", result_err);

}

// arrays need a constant length known at compile time, therefore type needs to be known too
/*
fn append_to_array(u: &mut [char], v: char) {
    let mut b: [char; u.len() + 1] = u + v;
}
*/

fn mutate(array: &mut [char; 3], pos: usize, v: char) {
    array[pos] = v;
    println!("{:?}", array);
}

fn mutate_locally(mut array: [char; 3], pos: usize, v: char) {
    array[pos] = v;
    println!("{:?}", array);
}

fn print_some_or_none(opt: Option<u8>) {
    // cool, match <T> { type } is possible :>
    match opt {
        Some(i) => println!("{}", i),
        None => println!("None"),
    }
}

fn arrow() -> u8 {
    return 8u8;
}
