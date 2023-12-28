
/*
    * SLICE:
        - reference to contiguous sequence of elements in a collection
        - provide a way to borrow part of a collection without taking
            ownersip of the entire collection
        - can be created from arrays, vectors, Strings and other collections
            implementing the Deref trait
*/


pub fn _slices(){
    println!("\n\n---- SLICES ---- \n\n");

    slice_how_to();
    size_of_slices();
    test_size();

    println!("\n\n---- //SLICES// ---- \n\n");
}

fn slice_how_to(){
    let arr: [i32; 4] = [1,2,3,4];

    let slice: &[i32] = &arr[2..4];

    println!("array: {:?}", arr);
    println!("array slice: {:?}", slice);

    let str: String = String::from("Hello");
    let str_slice: &str = &str[0..1];

    println!("String: {}", str);
    println!("String slice: {}", str_slice);

}

fn size_of_slices() {
    /*
        Every char weight 4 bytes so arr = 12 bytes
    */
    let arr: [char; 3] = ['中', '国', '人'];
    assert!(std::mem::size_of_val(&arr) == 12);

    /*
        A slice reference contains a pointer and a lenght ( both usize )
        so its size is always 16 bytes ( on x64-bit machines )
    */
    let slice: &[char] = &arr[..2];
    
    // TIPS: slice( reference ) IS NOT an array, if it was an array, then `assert!` will be passed: 
    //  Each of the two chars '中' and '国'  occupies 4 bytes, 2 * 4 = 8
    assert!(std::mem::size_of_val(&arr) == 12);
    assert!(std::mem::size_of_val(&slice) == 16);

    println!("Success!");
}

fn test_size(){

    /*
        Here you see that the char array size is 20 bytes
        and for any dimension of the char slice its size remains 16bytes
    */

    let char_arr: [char; 5] = ['a', 'b', 'c', 'd', 'e'];

    let char_slice : &[char] = &char_arr[..4];

    println!("char_arr: {:?} | size: {}", char_arr, std::mem::size_of_val(&char_arr)); // 20 bytes
    println!("char_slice: {:?} | size: {}", char_slice, std::mem::size_of_val(&char_slice));// 16 bytes
}