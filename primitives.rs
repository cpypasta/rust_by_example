#![allow(unused_variables)]

fn main() {
    // SCALAR TYPES

    // signed integers (with max values)
    // the number refers to the number of bits; 8 bits = 1 byte
    let x: i8 = 127; // -128
    let x: i16 = 32767; // -32768
    let x: i32 = 2147483647; // -2147483648
    let x: i64 = 9223372036854775807; // -9223372036854775808
    let x: i128 = 170141183460469231731687303715884105727; // -170141183460469231731687303715884105728
    let x: isize = 9223372036854775807; // -9223372036854775808 (pointer size)    

    println!("Signed integer bytes:");
    println!("i8_size: {}", std::mem::size_of_val(&23i8));
    println!("i16_size: {}", std::mem::size_of_val(&23i16));
    println!("i32_size: {}", std::mem::size_of_val(&23i32));
    println!("i64_size: {}", std::mem::size_of_val(&23i64));
    println!("i128_size: {}", std::mem::size_of_val(&23i128));
    println!("isize_size: {}", std::mem::size_of_val(&23isize));

    // unsigned integers (with max values)
    let x: u8 = 255; // 0; also how a byte is represented in Rust
    let x: u16 = 65535; // 0
    let x: u32 = 4294967295; // 0
    let x: u64 = 18446744073709551615; // 0
    let x: u128 = 340282366920938463463374607431768211455; // 0
    let x: usize = 18446744073709551615; // 0 (pointer size)

    println!("Unsigned integer bytes:");
    println!("u8_size (byte): {}", std::mem::size_of_val(&23u8));
    println!("u16_size: {}", std::mem::size_of_val(&23u16));
    println!("u32_size: {}", std::mem::size_of_val(&23u32));
    println!("u64_size: {}", std::mem::size_of_val(&23u64));
    println!("u128_size: {}", std::mem::size_of_val(&23u128));
    println!("usize_size: {}", std::mem::size_of_val(&23usize));

    // floating-point numbers
    let x: f32 = f32::MAX;
    println!("f32 w/ no fraction positive: {:.1}", x);
    let x: f32 = f32::MIN;
    println!("f32 w/ no fraction negative {:.1}", x);
    let x: f32 = 8388607.5;
    println!("f32 with single fraction positive {:.1}", x);
    let x: f32 = -8388607.5; // negative with fraction
    println!("f32 with single fraction negative {:.1}", x);
    let x: f32 = 260000.55;
    println!("f32 with double fraction positive {:.2}", x);
    let x: f32 = 99999.555;
    println!("f32 with triple fraction positive {:.3}", x);

    let x: f64 = f64::MAX;
    println!("f64 w/ no fraction positive: {:.1}", x);
    let x: f64 = f64::MIN;
    println!("f64 w/ no fraction negative: {:.1}", x);

    println!("Floating-point bytes:");
    println!("f32_size: {}", std::mem::size_of_val(&23f32));
    println!("f64_size: {}", std::mem::size_of_val(&23f64));

    // char unicode
    let x: char = 'a';
    println!("char: {}, char_bytes ascii: {}", x, b'a');

    // boolean
    let x: bool = true;
    println!("bool: {}", x);

    // unit type
    let x: () = ();
    println!("unit type: {:?}", x);


    // COMPOUND TYPES

    // tuples
    let x: (i32, f64, char) = (1, 2.0, 'a'); // can mix types
    println!("tuple: {:?}", x);

    // arrays
    let x: [i32; 5] = [1, 2, 3, 4, 5]; // must be same type
    println!("array: {:?}", x);


    // POINTERS
    let x: &String = &("boo".to_string());
    println!("pointer: {:p}", x);
    let x: String = "boo".to_string();
    println!("pointer: {:p}", x.as_ptr());


    // DEFAULTS
    let a = 1; // i32
    let a = 1.0; // f64
}