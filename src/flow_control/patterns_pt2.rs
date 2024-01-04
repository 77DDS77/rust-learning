pub fn _pattern_pt2(){

    println!("\n\n---- PATTERNS PT2 ---- \n\n");

    range_matching();
    destructuring_in_matching();
    exercise_at_operator();
    match_guard();
    first_and_last();
    mutable_value_match();

    println!("\n\n---- //PATTERNS PT2// ---- \n\n");
}

fn range_matching() {
    
    let n:i32 = 7;
    match_number(n);
    
}

fn match_number(n: i32) {
    match n {
        // Match a single value
        1 => println!("One!"),
        
        //? or conditional
        2 | 3 | 4 | 5 => println!("match 2 -> 5"),
        // Match an inclusive range
        6..=10 => {
            println!("match 6 -> 10")
        },
        _ => {
            println!("match -infinite -> 0 or 11 -> +infinite")
        }
    }
}

/*
    The @ operator lets us create a variable that holds a value, 
    at the same time we are testing that value to see whether it 
    matches a pattern.
*/

struct Point {
    x: i32,
    y: i32,
}

fn destructuring_in_matching() {

    let p: Point = Point { x: 3, y: 20 };

    match p {
        //? here I can't use 'y'
        Point { x, y:0 } => println!("\n1. On the x axis at {}", x),
        
        //? but I can use it if I use the '@' operator creating a variable to store the value
        Point { x, y: pippo@ 2 } => println!("\n1. On the x axis at {} and y {}", x, pippo),
        
        //? same here but I can match range or conditional statements
        Point { x: x@ 0..=5, y: y@ (10 | 20 | 30) } => println!("\n2. On the y axis at {} {}", y, x),

        //? Here I can use both because the is no matching pattern (?)
        Point { x, y } => println!("\n3. On neither axis: ({}, {})", x, y),
    }
}

enum Message {
    Hello { id: i32 },
}

// esercizio @ operator
fn exercise_at_operator() {
    let msg: Message = Message::Hello { id: 11 };

    match msg {
        Message::Hello { id: id@ 3..=7 } => println!("\nFound an id in range [3, 7]: {}", id),
        Message::Hello { id: newid@ (10 | 11 | 12) } => {
            println!("\nFound an id in another range [10, 12]: {}", newid)
        }
        Message::Hello { id } => println!("\nFound some other id: {}", id),
    }
}

/*
    A match guard is an additional if condition specified after 
    the pattern in a match arm that must also match, 
    along with the pattern matching, for that arm to be chosen.
*/
fn match_guard() {
    let num: Option<i32> = Some(4);
    let split: i32 = 5;
    match num {
         //? arm with additional if contion (match guard)
        Some(x) if x < split => {
            assert!(x < split);
            println!("\nx < split");
        },
        Some(x) => {
            assert!(x >= split);
            println!("\nx < split");
        },
        None => (),
    }

}

/* 
    you can ignore the remaining parts of the value with 
    '..' operator
*/
fn first_and_last() {
    let numbers = (2, 4, 8, 16, 32, 64, 128, 256, 512, 1024, 2048);

    match numbers {
        (first, .., last) => {
           assert_eq!(first, 2);
           assert_eq!(last, 2048);
           println!("\nFirst is {}, last is {}", first, last);
        }
    }

}

/*
    Using pattern &mut V to match a mutable reference requires you 
    to be very careful, due to V being a value after matching.
*/
fn mutable_value_match() {
    let mut v: String = String::from("hello,");
    let r: &mut String = &mut v;

    match r {
    //  &mut value => value.push_str(" world!") , //! I don't have to match the mutable reference

        value => { //? I have to match the value directly (wich is itself the mutable reference)
            value.push_str(" world!");
            println!("\n{}", value);
        } 
    }
}