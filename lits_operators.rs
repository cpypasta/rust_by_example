fn main() {
    println!("1 + 2 = {}", 1u32 + 2); // can cast literal number
    println!("1e4 is {}", 1e4); // scientific notation

    // boolean logic
    println!("true and false is {}", true && false);
    println!("true or false is {}", true || false);
    println!("not true is {}", !true);

    // integer literals
    println!("0xFF is {}", 0xFF); // hex
    println!("0o77 is {}", 0o77); // octal
    println!("0b1111 is {}", 0b1111); // binary
    println!("255 in hex is 0x{:X}", 255); // uppercase hex
    println!("255 in hex is 0x{}", format!("{:x}", 255)); // lowercase hex with format! macro
    println!("1_000_000 is {}", 1_000_000); // underscores for readability

    // bitwise operations
    println!("0011 AND 0101 is {:04b}", 0b0011 & 0b0101); // AND
    println!("0011 OR 0101 is {:04b}", 0b0011 | 0b0101); // OR
    println!("0011 XOR 0101 is {:04b}", 0b0011 ^ 0b0101); // XOR
    println!("1 << 5 is {}", 1 << 5); // shift left
    println!("0x80 >> 2 is 0x{:x}", 0x80 >> 2); // shift right
}