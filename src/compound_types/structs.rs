
/*
    * STRUCT:
        - basically the equivalent of a .ts interface
        - compound type to group together values of different type
            into a NAMED data structure
        - similar to tuples, but each value has a name so values can 
            be accessed through this name
*/

pub fn _structs(){

    println!("\n\n---- STRUCTS ---- \n\n");

    basic_struct_declaration();
    tuple_struct();
    destructuring_structured_tuple();
    debug_trait();
    

    println!("\n\n---- //STRUCTS// ---- \n\n");
}

struct Person {
    name: String,
    age: u8,
    email: String
}

fn basic_struct_declaration(){

    let p: Person = Person{
        name: String::from("Davide"),
        age: 26u8,
        email: String::from("asdsad@asd.it")
    };

    println!("Person:\n name: {}, age: {}, email {}\n", p.name, p.age, p.email);
    
    //I can make mutable struct
    let mut mut_p = create_person(String::from("Ajeje"), 32u8, String::from("ajeje@brazorf.it"));
    println!("Mutable pre mutation:\n name: {}, age: {}, email {} \n", mut_p.name, mut_p.age, mut_p.email);

    mut_p.name = String::from("Pdor");
    mut_p.email = String::from("subaru@baraca.com");

    println!("Mutable post mutation:\n name: {}, age: {}, email {} \n", mut_p.name, mut_p.age, mut_p.email);

}

fn create_person(name: String, age:u8, email:String) -> Person {
    Person{ name, age, email}
}

// * Tuple Structs
struct RgbColor(u8, u8, u8);

fn tuple_struct(){
    let white: RgbColor = RgbColor(255,255,255);

    println!("\n R: {}, G: {}, B: {}", white.0, white.1, white.2)
}


/*
    * Destructuring structured Tuple
    When destructuring a structured tuple you have
    to provide the type of struct. SEE *1
*/
struct Color(i32, i32, i32);

fn destructuring_structured_tuple() {
    let v: Color = Color(0, 127, 255);
    check_color(v);

    println!("Success! - destructuring_structured_tuple\n\n");
}   

fn check_color(p: Color) {
    let Color(x, _, z) = p; //? {*1}
    assert_eq!(x, 0);
    assert_eq!(p.1, 127);
    assert_eq!(z, 255);
}

/*
    * DEBUG TRAIT
    we can use it to make the struct printable via the {:?} debug notation
    and to print to the stderr via the dbg!(..) macro
*/

// Fill the blanks to make the code work
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

fn debug_trait() {
    let scale: u32 = 2;
    
    let rect1: Rectangle = Rectangle {
        width: dbg!(30 * scale), // Print debug info to stderr and assign the value of  `30 * scale` to `width`
        height: 50,
    };

    dbg!(&rect1); // Print debug info to stderr

    println!("\n\nrect1: {:?} \nWidth: {}\nHeight: {}", rect1, rect1.width, rect1.height); // Print debug info to stdout
}
