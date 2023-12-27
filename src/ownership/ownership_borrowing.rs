
/*
    *OWNERSHIP RULES:
        - each value in Rust has an owner
        - there can only be one owner at a time //!(1)
        - when the owner goes out of scope the value is dropped

    *MEMORY KNOWLEDGE:
        - pushing on the stack is faster than allocating on the heap
        because in the stack the location for the new data is always at the top 
        
    *HEAP:
        - types of unkwnown size will get allocated to the heap and pointer
        to the value is pushed to the stack, because a pointer is fixed size (usize) //!(2)

        - allocating to the heap returns a pointer - address to the 
        location where the data is been allocated

    *STRINGS:
        - a differenza dei numberi (che hanno una size definita i32/i62/f32/..)
        Strings are mutable, size can change at runtime
        - Strigns are stored with a pointer in the stack that points to an address
        on the heap where the value is stored

        let s1 = String::from("Hello");

        s1 contains:
            - the pointer to the heap where the value is stored
            - the length of the string (size in bytes)
            - the capacity - total amount of memory recieved from the allocator

        These three values are of size usize *(2) so in case of a x64-bit machine
        is 3 * 8 bytes = 24 bytes

        * STRING LITERALS:
        A string literal is a string of fixed size and immutable

        let str_lit: &str = "hello";
        let copy = str_lit;

        Being of fixed size they are saved on the stack and can be copied instead of cloned or moved
        
    *COPY vs MOVE
        * COPYING:
        
            let x:i32 = 32;
            let y = x;

            The value x being an i32 is FIXED SIZE so it is stored on the stack.
            Copying on the stack is CHEAP so when I assign its value to the variable
            y it gets copyied on the stack and two separate values will be stored.
        
        * MOVING:

            let s1 = String::from("Hello");
            let s2 = s1;

            The vatiable s1 being a string is NOT OF FIXED SIZE so the value is stored to
            the heap and the pointer is stored on the stack. When copying it to the variable
            s2 ONLY THE POINTER GETS COPIED, not the whole data.
            !BUT THIS VIOLATES THE SECOND RULE *(1)
            There can only be one owner to a value so s1 gets dropped

                *DEEP CLONING
                To specifally tell the copilier that we want the value to be copied and not the pointer

                    let s1 = String::from("Hello");
                    let s2 = s1.clone();

                Now two pointer are stored separatly in the stack for the values of s1 and s2

*/

pub fn _ownership_and_borrowing(){
    println!("\n\n---- OWNERSHIP AND BORROWING ---- \n\n");

    let s1: String = String::from("Hello"); //the ownership of the value is s1

    //being a String (not fixed size) the ownersip gets passed to the function
    takes_ownership(s1); //when the scope is executed s1 is dropped
    // println!(s1); can't 

    exercise_1();
    exercise_2();
    exercise_3();
    exercise_4();

    println!("\n\n---- //OWNERSHIP AND BORROWING// ---- \n\n");
}

fn takes_ownership(some_string: String) {
    println!("I've got the string: {}", some_string);
}

/* Exercises */

fn exercise_1() {
    let s: String = give_ownership();
    println!("{} - complete", s);
}

fn give_ownership() -> String {
    let s = String::from("Exercise - 1");
    s
}

//

fn _just_for_knowledge() -> String {
    let s: String = String::from("hello, world");

    // s.into_bytes() convert String to a vector of bytes (numbers that represents each character in an Array)
    //* but s.into_bytes() consumes the string so s gets dropped
    //* use s.as_bytes() that do not consume the string ( it takes a reference to the string )
    let _s: &[u8] = s.as_bytes();
    s
}

/*
    * REFERENCES
*/
fn exercise_2() {
    let s: String = String::from("Exercise - 2");

    print_str(&s); // when passing a reference the owner doesn't change so s isn't dropped

    println!("{} - completed", s);
}

fn print_str(s: &String)  {
    println!("{}",s)
}

//

fn exercise_3() {
    let s: String = String::from("hello, "); //? non mutable
    
    // Mutability can be changed when ownership is transferred.
    let mut s1: String = s; //? mutable

    s1.push_str("world");

    println!("{} - EX 3 completed!", s1);
}

/*
    * BOXING
    Boxing means forcing the compiler to save on the heap
    so in this case the variable x contains a pointer to is value on the heap

    i32 (fixed size) normally gets saved in the stack 
*/
fn exercise_4() {
    let x: Box<i32> = Box::new(5);
    
    // let ...      // Implement this line, dont change other lines!
    let mut y: Box<i32> = Box::new(1);
    
    // the star operator is used to access the value of a Boxed variable
    // y = 4 non posso farlo perche assegnerei un i32 ad una variabile che contiene un puntatore
    *y = 4;
    
    assert_eq!(*x, 5);

    println!("Exercise 4 - completed");
}