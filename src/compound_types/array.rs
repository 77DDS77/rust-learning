
/*
    * ARRAY:
        - an array is a FIXED SIZE collection of elements
            of the same data type stored as contiguous block 
            in the STACK MEMORY (because its size is known at compile time)
        - arrays can neither grow or shrink, they must retain their size
        - the signature of an array type is 
            * [T; len]
            Where T is the type of the data it contains and len is the lenght of the array
        
*/

pub fn _arrays(){
    println!("\n\n---- ARRAYS ---- \n\n");

    array_declaration();
    exercise_1_array_size();
    array_auto_fill();
    exercise_2();

    println!("\n\n---- //ARRAYS// ---- \n\n");
}

fn array_declaration() {
    let arr: [i32; 5] = [1, 2, 3, 4, 5];

    assert!(arr.len() == 5);

    println!("Success - arr = {:?}", arr);
}

fn exercise_1_array_size(){
    // We can ignore parts of the array type or even the whole type, let the compiler infer it for us
    let arr0 = [1, 2, 3, 4, 5];
    let arr: [_; 3] = ['a', 'b', 'c'];
    
    // Arrays are stack allocated, `std::mem::size_of_val` returns the bytes which an array occupies
    // A char takes 4 bytes in Rust: Unicode char
    assert!(std::mem::size_of_val(&arr) == 12);
    println!("char array {:?} (4 bytes x 3 char): { }bytes",arr, std::mem::size_of_val(&arr));
    println!("i32 array {:?} (i32 x 5) {} bytes", arr0, std::mem::size_of_val(&arr0));
}

fn array_auto_fill() {
    /*
        declaring an array with [1; 100] i'm saying that I want
        the value 1 assigned to 100 slots
    */
    let list: [i32; 100] = [1; 100] ;

    assert!(list[0] == 1); //? accessing to a single value
    assert!(list.len() == 100); //? getting the length of an array
    println!("\n arrays of 100 ones: {:?}\n", list);
}

// Fix the error
fn exercise_2() {
    let names: [String; 2] = [String::from("Sunfei"), "Sunface".to_string()];
    
    // `Get` returns an Option<T>, it's safe to use
    let _name0: &String = names.get(0).unwrap();

    // But indexing is not safe
    let _name1: &String = &names[1];

    println!("{} {}", _name0, _name1);
}