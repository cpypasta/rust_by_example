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

    // unsigned integers (with max values)
    let x: u8 = 255; // 0
    let x: u16 = 65535; // 0
    let x: u32 = 4294967295; // 0
    let x: u64 = 18446744073709551615; // 0
    let x: u128 = 340282366920938463463374607431768211455; // 0
    let x: usize = 18446744073709551615; // 0 (pointer size)

    // floating-point numbers
    let x: f32 = 340282346638528859811704183484516925440.0;
    println!("f32 w/ no fraction positive: {:.1}", x);
    let x: f32 = -34028234663852885981170418348451692544.0;
    println!("f32 w/ no fraction negative {:.1}", x);
    let x: f32 = 8388607.5;
    println!("f32 with single fraction positive {:.1}", x);
    let x: f32 = -8388607.5; // negative with fraction
    println!("f32 with single fraction negative {:.1}", x);s
    let x: f32 = 260000.55;
    println!("f32 with double fraction positive {:.2}", x);
    let x: f32 = 99999.555;
    println!("f32 with triple fraction positive {:.3}", x);

    let x: f64 = 17976931348623157580412819756850388593900235011794141176754562789180111453639664485361928830517704263393537268510363518759043843737070229269956251768752166883397940628862983287625967246810352023792017211936260189893797509826303293149283469713429932049693599732425511693654044437030940398714664210204414967808.0;
    println!("f64 w/ no fraction positive: {:.1}", x);
    let x: f64 = -17976931348623157580412819756850388593900235011794141176754562789180111453639664485361928830517704263393537268510363518759043843737070229269956251768752166883397940628862983287625967246810352023792017211936260189893797509826303293149283469713429932049693599732425511693654044437030940398714664210204414967808.0;
    println!("f64 w/ no fraction negative: {:.1}", x);

    // char unicode
    let x: char = 'a';
    println!("char: {}", x);

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