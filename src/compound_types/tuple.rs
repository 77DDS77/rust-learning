
/*
    * TUPLES:
        - way to store related pieces of information in a single variable
        - collection of calues of different type grouped together as
            a single COMPOUND TYPE ( type composed of other types )
        - stored as a fixed size contiguous block of memory in the stack
        - signature is (T1, T2, T3 ..)
*/

pub fn _tuples(){

    println!("\n\n---- TUPLES ---- \n\n");

    tuple_declaration();
    accessing_tuple_member();
    too_long_tuple();
    destructuring();
    tuples_as_function_argument();


    println!("\n\n---- //TUPLES// ---- \n\n");

}


fn tuple_declaration() {
    let _t0: (u8,i16) = (0, -1);

    // Tuples can be tuple's members
    let _t1: (u8, (i16, u32)) = (0, (-1, 1));

    // Fill the blanks to make the code work
    let t: (u8, u16, i64, &str, String) = (1u8, 2u16, 3i64, "hello", String::from(", world"));

    println!("Tuple: {:?}", t);
}

fn accessing_tuple_member() {
    let t: (&str, &str, &str) = ("i", "am", "sunface");
    assert_eq!(t.2, "sunface");

    println!("Success! t.2: {} == sunface", t.2);
}

fn too_long_tuple() {
    let _too_long_tuple = (1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13);
    // println!("too long tuple: {:?}", too_long_tuple); //!can't, max len is 12
    let max_size_tuple = (1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12);
    println!("max size tuple: {:?}", max_size_tuple);
}

fn destructuring() {
    let tup: (i32, f64, &str) = (1, 6.4, "hello");

    // non devo assegnare i tipi, lo fa in automatico
    let (x, z, y) = tup;

    assert_eq!(x, 1);
    assert_eq!(y, "hello");
    assert_eq!(z, 6.4);

    println!("Success! - destructuring");
}

/*
    * Tuples can be used as function arguments and return values
*/
fn tuples_as_function_argument() {
    let (x, y) = sum_multiply((3,2));

    assert_eq!(x, 5);
    assert_eq!(y, 6);

    println!("Success! x: {} y: {}", x, y);
}

fn sum_multiply(nums: (i32, i32)) -> (i32, i32) {
    (nums.0 + nums.1, nums.0 * nums.1)
}