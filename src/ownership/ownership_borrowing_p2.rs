
/*
    * BORROWING:
        - way of temporarly access data without taking ownership of it
        - when borrowing you are taking a reference (pointer) to the
            data, not the data itself.
        - data can be borrowed mutably or immutably

        ! RULES OF BORROWING
        - you can have either:
            - ONE mutable reference
                or
            - ANY number of immutable references
        - references MUST always be valid



*/

pub fn _ownership_and_borrowing_p2(){
    println!("\n\n---- OWNERSHIP AND BORROWING P2 ---- \n\n");


    borrowing_reference_example();
    mutable_reference_example();
    printing_memory_address();
    accessing_pointer_value();
    exercise_1();

    println!("\n\n---- //OWNERSHIP AND BORROWING P2// ---- \n\n");
}

fn borrowing_reference_example(){

    let s1: String = String::from("Hello");

    let len: usize = calculate_lenght(&s1);

    println!("String: {} | Size: {}", s1, len);

}

fn calculate_lenght(s: &String) -> usize {
    s.len()
}

/*
    !NB:
    notare come l'onwersip del valore rimane alla variabile s,
    la funzione change(...) non restituisce nessun valore,
    modifica e basta
*/
fn mutable_reference_example(){
    let mut s: String = String::from("BEFORE");
    println!("Prima: {}", s);

    change(&mut s);
    println!("Dopo: {}", s);
}

fn change(some_string: &mut String){
    some_string.push_str(" - AFTER");
}

/*
    I can borrow only one mutable reference at a time
*/
fn _multiple_mutable_refencences(){
    let mut s: String = String::from("Hello");
    
    let _r1 = &mut s;
    // let r2 = &mut s; //!cannot borrow `s` as mutable more than once at a time
}

/*
    If I have any immutable reference I can not have any mutable references
*/
fn _multiple_refences(){

    let mut s: String = String::from("Hello");

    let r1: &String = &s;
    let r2: &String = &s;
    // let r3: &String = &mut s; //!cannot borrow `s` as mutable because it is also borrowed as immutable

    println!("{}{}", r1, r2);

    //BUT

    /*
        If I don't use the immutable references anymore I can create a mutable reference.
    */

    let r3_bis = &mut s;

    println!("{}", r3_bis);

}


/*
    * ESERCIZI
*/

//* Printing the memory address
fn printing_memory_address() {
    let x: i32 = 5;
    let p: &i32 = &x; //p e' una reference di un i32
 
    // notation {:p} e' per stampare l'indirizzo in memoria
    println!("the memory address of x is {:p}", p); // One possible output: 0x16fa3ac84
    
    // test con String
    let s: String = String::from("Hello");
    let s_ref: &String = &s;
    println!("the memory address of String is {:p}", s_ref);
}

fn accessing_pointer_value(){
    let x: i32 = 5;
    let y: &i32 = &x;

    // symbol * = dereferencing operator | accede al valore del pointer
    assert_eq!(5, *y);

    println!("Value of the ponter y ({:p}) = {}", y, *y);  
}

fn exercise_1() {
    let mut s: String = String::from("hello, ");

    push_str(&mut s);
    println!("{}",s); // hello, world

    println!("Success - Exercise 1!");
}

fn push_str(s: &mut String) {
    s.push_str("world");
}