use u8;
use u16;
use u32;
use u64;
use u128;

use i8;
use i16;
use i32;
use i64;
use i128;

use f32;
use f64;

use bool;
use char;

fn main() {
    let boolean: bool = true;
    let c: char = '-';
    let mut b8: [u8; 2] = [0, 2];
    let mut b16: [u16; 2] = [0, 2];
    let s = "just a rusty";

    println!("unsigned integers:");
    println!("[{}...{}] bits:{}", u8::MIN, u8::MAX, u8::BITS);
    println!("[{}...{}] bits:{}", u16::MIN, u16::MAX, u16::BITS);
    println!("[{}...{}] bits:{}", u32::MIN, u32::MAX, u32::BITS);
    println!("[{}...{}] bits:{}", u64::MIN, u64::MAX, u64::BITS);
    println!("[{}...{}] bits:{}", u128::MIN, u128::MAX, u128::BITS);
    println!();
    println!("signed integers:");
    println!("[{}...{}] bits:{}", i8::MIN, i8::MAX, i8::BITS);
    println!("[{}...{}] bits:{}", i16::MIN, i16::MAX, i16::BITS);
    println!("[{}...{}] bits:{}", i32::MIN, i32::MAX, i32::BITS);
    println!("[{}...{}] bits:{}", i64::MIN, i64::MAX, i64::BITS);
    println!("[{}...{}] bits:{}", i128::MIN, i128::MAX, i128::BITS);
    println!();
    println!("floating point:");
    println!("[{}...{}]", f32::MIN, f32::MAX);
    println!("[{}...{}]", f64::MIN, f64::MAX);
    println!();
    println!("boolean:");
    println!("bool = {}", boolean.to_string());
    println!("true | false = {}", true | false);
    println!();
    println!("char:");
    println!("c = {}", c);
    println!("isalphabetic: {}", c.is_alphabetic());
    println!("isnumeric: {}", c.is_numeric());
    println!("isalphanumeric: {}", c.is_alphanumeric());
    let b8_result = c.encode_utf8(&mut b8);
    let b16_result = c.encode_utf16(&mut b16);
    println!("b8 buffer: {:#}", b8_result);
    println!("b16 buffer: {:#?}", b16_result);
    println!();
    let s_ptr = s.as_ptr();
    unsafe {
        println!("{:?} {:?}", s_ptr, *s_ptr);
        let s_ptr_i = s_ptr.cast_mut();
        println!("{:?} {:?}", s_ptr, s_ptr_i);
        let s_ptr_i2 = s_ptr_i.add(1);

        let value_of_s_ptr_i2 = *s_ptr_i2;
        println!("{:#?}", value_of_s_ptr_i2);
    }
}
