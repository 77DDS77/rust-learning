
pub fn _flow_control(){

    println!("\n\n---- FLOW CONTROL ---- \n\n");

    if_else();
    if_else_assignement();
    for_loop_using_range();
    for_loop_using_array();
    for_loop_with_index();
    break_continue();
    while_loop();
    rust_loop();
    rust_loop_assignement();
    nested_loops();

    println!("\n\n---- //FLOW CONTROL// ---- \n\n");
}

/*
    * IF ELSE
*/
fn if_else() {
    let n: i32 = 5;

    if n < 0 {
        println!("{} is negative\n", n);
    } else if n > 0 {
        println!("{} is positive\n", n);
    } else {
        println!("{} is zero\n", n);
    }
} 


// Fix the errors
fn if_else_assignement() {
    let n: i32 = 25;

    let big_n: i32 =
        if n < 10 && n > -10 {
            println!(", and is a small number, increase ten-fold");

            10 * n
        } else {
            println!(", and is a big number, halve the number");

            n / 2 as i32 //? cast it to i32 or else you have to use f32/f64
                         //? ma lo arrotonda per difetto
        };

    println!("{} -> {}\n", n, big_n);
}

/*
    * FOR 
*/

/*
    RANGE TIPS:
        0..100 will stop at 99
        0..=100 will stop at 100

    EXTRA:
        Instant is used to get the time in milliseconds
*/
use std::time::Instant;

fn for_loop_using_range(){
    let time_initial: Instant = Instant::now();

    for n in 0..=1_000 {
        print!("{}, ", n);
        // if n == 100 {
        //     println!("100")
        // }
    }

    let time_final: Instant = Instant::now();
    let duration: std::time::Duration = time_final.duration_since(time_initial);

    println!("Success!");
    println!("Time elapsed in for loop is: {:?}\n\n", duration);
}

fn for_loop_using_array() {
    let names: [String; 2] = [String::from("liming"),String::from("hanmeimei")];

    //? se non uso la reference variables gets moved 
    //? and names[] looses its ownership
    for name in &names {
        // Do something with name...
        println!("name = {}", name);
    }
    
    println!("names array = {:?}", names);
    
    let numbers: [i32; 3] = [1, 2, 3];
    // The elements in numbers are Copy，so there is no move here
    for n in numbers {
        // Do something with name...
        println!("n = {}", n);
    }
    
    println!("numbers = {:?}\n", numbers);
}

/*
    To obtain an ( index, value ) tuple out of a for loop
    we have to trasform the Array/Vector into an iterator
    using .into() (or other similar .into_mut() .into_iter())
    and then we .enumerate() it.

    values in the ( index, value ) tupla are always reference
    so remember that to access the value you have to dereference it
    ( using * operator )
*/
fn for_loop_with_index() {
    let a: [i32; 4] = [4, 3, 2, 1];
    let names: [String; 3] = [String::from("liming"),String::from("hanmeimei"),String::from("davide")];

    // Iterate the indexing and value in 'a'
    for (i,v) in a.iter().enumerate() {
        println!("Index: {} Value: {}",i ,v);
    }
    print!("arr: {:?}\n\n", a);
    
    for (i, v) in names.iter().enumerate(){
        println!("Index: {} Value: {}",i ,v);
        
    }
    print!("arr: {:?}\n\n", names);

    let mut b: [i32; 4] = [1,2,3,4];
    println!("\nMutable arr: {:?}", b);
    
    for (i, v) in b.iter_mut().enumerate() {
        *v += i as i32; //? remember to dereference since v is always a reference
    }
    
    println!("\nMutated arr: {:?}\n", b);

}

/*
    As every other laanguage
*/
fn break_continue() {
    let mut n: i32 = 0;
    for _i in 0..=100 {
       if n != 66 {
           n+=1;
           continue;
       }
       
       break;
    }

    assert_eq!(n, 66);

    println!("\nn = {} = 66 - Success!\n", n);
}


/*
    * WHILE LOOP
*/
fn while_loop() {
    // A counter variable
    let mut n: i32 = 1;

    // Loop while the condition is true
    while n < 10 {
        if n % 15 == 0 {
            println!("fizzbuzz");
        } else if n % 3 == 0 {
            println!("fizz");
        } else if n % 5 == 0 {
            println!("buzz");
        } else {
            println!("{}", n);
        }


        n += 1; // increment the counter
    }

    println!("n reached {}, so loop is over",n);
}


/*
    * LOOP
    Basically its an infinite loop that you manually have 
    to stop from its flow
*/
fn rust_loop() {
    let mut count: u32 = 0u32;

    println!("\n\nLet's count until infinity!");

    // Infinite loop
    loop {
        count += 1;

        if count == 3 {
            print!(" three,");

            // Skip the rest of this iteration
            continue;
        }

        print!(" {},", count);

        if count == 5 {
            println!("\nOK, that's enough");

            // exit from the infinite loop
            break;
        }
    }

    assert_eq!(count, 5);

    println!("rust loop stopped - Success!\n");
}


/*
    Rust loop can be used to assign values to variables
*/
fn rust_loop_assignement() {
    let mut counter: i32 = 0;

    let result: i32 = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2; //? syntax to return value on the break statement
        }
    };

    assert_eq!(result, 20);

    println!("Result ({}) == 20 - Success!\n\n", result);
}

/*
    It's possible to break or continue outer loops when dealing with nested loops. 
    In these cases, the loops must be annotated with some 'label, 
    and the label must be passed to the break/continue statement.
*/
fn nested_loops() {
    let mut count: i32 = 0;
    'outer: loop {
        println!("Outer loop -> count = {}", count);
        'inner1: loop {
            println!("Inner 1 loop -> count = {}", count);
            if count >= 20 {
                // This would break only the inner1 loop
                println!("Inner 1 loop -> exiting - count ({}) >= 20", count);
                break 'inner1; // `break` also works.
            }
            count += 2;
        }
        
        count += 5;
        
        '_inner2: loop {
            println!("Inner 2 loop -> count = {}", count);
            if count >= 30 {
                println!("Inner 2 loop -> exiting - count ({}) >= 30", count);
                // This breaks the outer loop
                break 'outer;
            }

            // This will continue the outer loop
            continue 'outer;
        }
    }

    assert!(count == 30);

    println!("Success!");
}


