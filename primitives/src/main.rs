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

fn main() {
    println!("[{}...{}] bits:{}", u8::MIN, u8::MAX, u8::BITS);
    println!("[{}...{}] bits:{}", u16::MIN, u16::MAX, u16::BITS);
    println!("[{}...{}] bits:{}", u32::MIN, u32::MAX, u32::BITS);
    println!("[{}...{}] bits:{}", u64::MIN, u64::MAX, u64::BITS);
    println!("[{}...{}] bits:{}", u128::MIN, u128::MAX, u128::BITS);

    println!("[{}...{}] bits:{}", i8::MIN, i8::MAX, i8::BITS);
    println!("[{}...{}] bits:{}", i16::MIN, i16::MAX, i16::BITS);
    println!("[{}...{}] bits:{}", i32::MIN, i32::MAX, i32::BITS);
    println!("[{}...{}] bits:{}", i64::MIN, i64::MAX, i64::BITS);
    println!("[{}...{}] bits:{}", i128::MIN, i128::MAX, i128::BITS);
}
