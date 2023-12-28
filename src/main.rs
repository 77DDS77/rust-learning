
mod numbers;
use numbers::integer as num_int;
use numbers::float as num_float;
use numbers::numbers_various as num_var;

mod data_types;
use data_types::char_bool_units as cbu;
use data_types::statement_expresion as stat_exp;

mod ownership;
use ownership::ownership_borrowing as own_bor;
use ownership::ownership_borrowing_p2 as own_bor_p2;

mod compound_types;
use compound_types::string_string_literal as s_lit;
use compound_types::array as arr;
use compound_types::slice as slc;

fn main() {
    let mut x: i32 = 5;
    let y: i32 = 12;

    {
        x += y;
        print!("x {x} | y {y}\n");
    }
    print!("x {x} | y {y}\n");
    println!("success\n");

    string_definition();
    println!("------------------------");
    // _shadowing();
    // destructuring();
    // _destructuring_assignments();

    num_int::_integers();
    num_float::_float();
    num_var::_numbers_various();

    cbu::_char_bools_units();
    stat_exp::_statement_and_expressions();

    own_bor::_ownership_and_borrowing();
    own_bor_p2::_ownership_and_borrowing_p2();

    s_lit::_string_and_string_literals();

    arr::_arrays();

    slc::_slices();
    
}

/**
    String definition (string literal)
*/
fn string_definition() {
    let poop: &str = "morning";
    println!("Good {}", poop)
}

/**
   Defing a variable with the same name of another variable
   but with a different value, the reference to that value
   exist only in it's scope.
*/
fn _shadowing() {
    let x: i32 = 12;
    {
        let x: i32 = 5;
        println!("x scope 2: {}", x); // x = 5
        assert_eq!(x, 5); //true
    }

    println!("x scope 1: {}", x); // x = 12
    assert_eq!(x, 12); //true

    let x: i32 = 42;
    println!("{}", x); // Prints "42".
}

/**
   Destructuring tuple.
   //? TUPLE: non mutable (in lenght) groupings  of variables (don't have to be of the same type)
*/
fn _destructuring() {
    let (mut x, y) = (1, 2);
    x += 2;

    assert_eq!(x, 3);
    assert_eq!(y, 2);

    println!("Success - destructuring!");
}

fn _destructuring_assignments() {
    
    // Tuple Destructuring
    let tup: (i32, &str, f64) = (10, "Rust", 3.14);
    println!("{:?}", tup);
    println!("{}", tup.0);
    let (a, b, c): (i32, &str, f64) = tup;

    println!("a: {}, b: {}, c: {}", a, b, c); // prints: a: 10, b: Rust, c: 3.14

    // Array Destructuring
    let arr: [i32; 5] = [1, 2, 3, 4, 5];
    println!("{:?}", arr);
    let [first, second, .., last]: [i32; 5] = arr;

    println!("first: {}, second: {}, last: {}", first, second, last); // prints: first: 1, second: 2, last: 5
}

