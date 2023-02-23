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
    println!("{:?}", array)
}