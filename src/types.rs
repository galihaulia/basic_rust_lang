/*
Primitive Types--
Integers: u8, i8, u16, i16, u32, i32, u64, i64, u128, i128 (number of bits they take in memory)
Floats: f32, f64
Boolean (bool)
Characters (char)
Tuples
Arrays
*/

pub fn types_res(){
    //Default is "i32"
    let x = 1;
    let x1 = 2;
    //Default is "f64"
    let y = 2.5;

    //add explicit type
    let z: i64 = 10101010101010;

    //Find max size
    println!("Max u8: {}", std::u8::MAX);
    println!("Max i8: {}", std::i8::MAX);
    println!("Max u16: {}", std::u16::MAX);
    println!("Max i16: {}", std::i16::MAX);
    println!("Max u32: {}", std::u32::MAX);
    println!("Max i32: {}", std::i32::MAX);
    println!("Max u64: {}", std::i64::MAX);
    println!("Max i64: {}", std::i64::MAX);
    println!("Max u128: {}", std::i128::MAX);
    println!("Max i128: {}", std::u128::MAX);
    
    //Boolean
    let is_active: bool = true;

    //Boolean get expretion
    let is_condition: bool = 10 < 5;
    let is_x = x + x1;
    let is_y = x - x1;
    let _is_res: bool = is_x != is_y;

    //Char
    let a1 = 'a'; //=>'ab' error
    let _a2 = "a"; //=>"ab" not
    //emoji
    let _face = '\u{1F600}';

    println!("{:?}", (x, y, z, is_active, is_condition, _is_res, a1, _a2, _face));
}