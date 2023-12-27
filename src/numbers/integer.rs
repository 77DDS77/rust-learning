
/**
 INTEGERS:
    singed integers (i*): can be both positive and negative
    unsigned integers (u*): can ONLY be POSITIVE
    8-bit	i8	u8 | i8 => -128 to 128 | u8 => 0 to 255
    16-bit	i16	u16 | i16 => -32767 to 32767 | u16 => 0 65535
    32-bit	i32	u32 |
    64-bit	i64	u64
    128-bit	i128	u128
    arch	isize	usize

    arch uses the system architecture, so for ex. x64 window system will use i64 or u64
    On a 64-bit machine, for example, usize is 64 bits, aligning with the word size of the machine.
     This means that operations involving usize values are typically efficient because they match 
     the natural size of machine words. 
 */
pub fn _integers(){
    println!("\n\n---- INTEGERS ---- \n\n");
    
    basic_declaration();
    max_constant();
    _checked_additions(254);

    println!("\n\n---- //INTEGERS// ---- \n\n");
}

fn basic_declaration(){
    let x: i32 = 8;
    let mut y: i32 = 5;
    println!("y: {}", y);

    y = x;
    
    let _z = 10; // Type of z ? 

    println!("y: {}", y);
    
    //? 38_u8 => value directly typed
    // since v expect a u16 I can infer u16 to the value *_u8
    // typing an integer as another integer
    let v: u16 = 38_u8 as u16;
    println!("v: {}", v);
    
    let x:u32 = 5;
    assert_eq!("u32".to_string(), type_of(&x));
    
    println!("Success!");
}

// Get the type of given variable, return a string representation of the type  , e.g "i8", "u8", "i32", "u32"
fn type_of<T>(_: &T) -> String {
    format!("{}", std::any::type_name::<T>())
}

/**
 let max_i8: i8 = i8::MAX
 using this annotation we assign the max value of the assigned type
 */
pub fn max_constant(){
    let max_i8: i8 = i8::MAX;
    let max_u8: u8 = u8::MAX;
    println!("MAX i8: {}", max_i8);
    println!("MAX u8: {}", max_u8);
}

/*
    Basically if you have a variable like u8 254 (max is 255) and try to add
    another u8 > 1 you cannot store it in a u8 variable, you need a u16.
    To do so you have to cast one of the two varables as u16 or you get an error.
*/
pub fn _checked_additions(n: u8){
    let v1: u16 = n as u16 + 8;
    let v2 = u16::checked_add(n as u16, 8).unwrap();
    println!("v1:{}, v2:{}", v1, v2);
}

fn _other_numbers_rappresentation(){
    // 1_024 esadecimale
    // 0xff
    // 0o77
    // 0b1111_1111 binario
    let v = 1_024 + 0xff + 0o77 + 0b1111_1111; // 1024 + 255 + 63 + 255
    assert!(v == 1597);
    println!("success.")
}
