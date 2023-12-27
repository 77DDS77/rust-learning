
use std::mem::size_of_val; //* returns the size in bytes

pub fn _char_bools_units(){
    println!("\n\n---- CHAR BOOLS UNITS ---- \n\n");

    char_basic_declaration();
    bool_basic_declaration();
    unit_type();

    println!("\n\n---- //CHAR BOOLS UNITS// ---- \n\n");
}

/*
    Come in Java, si usano gli apici singoli per i char e i 
    doppi apici per le stringe.
*/
fn char_basic_declaration() {
    let c1: char = 'a';
    println!("Size of c1: {} bytes", size_of_val(&c1));

    let c2: char = '中';
    println!("Size of c2: {} bytes", size_of_val(&c2));
}

fn bool_basic_declaration(){
    let x: bool = true;
    let y: bool = false;

    if x || y {
        println!("Boo-l")
    }
}

/*
    A UNIT TYPE is a variable that doesn't hold any value, it's size is 0 bytes.
    Used for example if a function doesn't return a value it returns a unit type.

    Avviene implicitamente (tipo :void in ts) quindi non e' obbligatorio annotarlo.
*/
fn unit_type() -> () {
    let unit_type: () = ();
    println!("Unit type size: {} bytes.",size_of_val(&unit_type));
    println!("I don't return anything so I return a unit type ()!")
}