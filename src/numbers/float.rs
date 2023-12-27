
/*
    TYPE OF FLOAT
    - f32
    - f64 -> DEFAULT
*/
pub fn _float(){
    println!("\n\n---- FLOATS ---- \n\n");
    
    basic_declaration();
    exercise_1();

    println!("\n\n---- //FLOATS// ---- \n\n");
}

/*
    1_000.000_1 => questa sintassi e' tipo pyhton, l'underscore
    serve solo per la readabilty
*/
fn basic_declaration(){
    let x: f64 = 1_000.000_1; 
    let y: f32 = 0.12; 
    let z: f64 = 0.01_f64;

    println!("Success! {} {} {}", x, y, z);
}

/*
    CONSIDERATION:
    this is to test floating point precision (like the decimal additions problem in JS)
    0.1 + 0.2  with the default f64 will be equals to 0.300000000002, so we use f32
    ( a less precise number ) in order to assert true when doing 0.1_f32 + 0.2_f32 == 0.3_f32
*/
fn exercise_1() {
    //make it execute in two ways
    // assert! => execute the predicate
    //? test case: assert!(0.1 + 0.2 == 0.3);
    
    assert!(0.1_f32 + 0.2_f32 == 0.3_f32);
    assert!(0.1 as f32 + 0.2 as f32 == 0.3 as f32);
    // assert!(0.1 as f64 + 0.2 as f64 == 0.3 as f64); => this will PANIC (throw error)

    println!("Survived!");
}