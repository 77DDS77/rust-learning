
pub fn _string_and_string_literals(){
    println!("\n\n---- STRING AND STRING LITERALS ---- \n\n");

    replace_string();
    string_concat();
    str_slice_to_string();
    accessing_substrings();
    iterating_a_string();

    println!("\n\n---- //STRING AND STRING LITERALS// ---- \n\n");
}

/*
    .replace() => 
        - creates a new String, and copies the data from this string slice into it 
        - does not consume the string
*/
fn replace_string() {
    let s: String = String::from("I like dogs");
    println!("{}", s); // I like dogs
    
    let s1: String = s.replace("dogs", "cats");

    println!("{}", s); // I like dogs
    println!("{}", s1); // I like cats
}

/*
    * CONCATENATION +
    * String -> to &str(string slice)
*/
fn string_concat() {
    let s1: String = String::from("hello,");
    let s2: String = String::from("world!");

    /*
        When concatenating with the + operator
        the FIRST argument must be of type String while
        ALL THE OTHER have to be of type &str.

        Use .as_str() to pass a String as &str
    */
    let s3: String = s1 + s2.as_str(); 
    // println!("s1: {}", s1); //! cant because value moved to s3
    println!("s2: {}", s2);
    println!("s3: {}", s3);
}

/*
    * &str (string slice) -> to String
*/
fn str_slice_to_string() {
    let s: &str = "hello, world";
    greetings(s.to_string()); //? method 1
    greetings(String::from(s)); //? method 2
}

fn greetings(s: String) {
    println!("{}", s)
}

fn accessing_substrings() {
    let s1: String = String::from("hi,中国");
    let h: &str = &s1[0..1]; //? Tips: `h` only takes 1 byte in UTF8 format
    assert_eq!(h, "h");
    println!("&s1[0..1] {}", h);
    
    let h1: &str = &s1[3..6]; //? Tips: `中`  takes 3 bytes in UTF8 format
    assert_eq!(h1, "中");
    println!("&s1[3..6] {}", h1);

    println!("Success!");
}

/*
    To iterate a string use .chars() to put it in an iterator
*/
fn iterating_a_string() {

    for c in "你好，世界".chars() {
        println!("{}", c)
    }
}
