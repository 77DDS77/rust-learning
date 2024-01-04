
/*
    * PATTERN MATCHING
        - powerful cosntruct that allows to compare 
            a set of patterns then execute different code
            based on wich pattern matches
        - patterns can be made up of literal values, variable names,
            wildcards, ecc..
        - in a match statement all possible cases must be handled, it's
            enforced by the compiler
*/

pub fn _pattern_match(){

    println!("\n\n---- PATTER MATCH ---- \n\n");

    match_to_return_value();
    matching_default_case();
    match_with_logic_operators();
    assignement_with_match();
    accessing_enum_data_via_match_statement();
    matches_macro();

    exercise_1();
    exercise_2();
    exercise_3();
    exercise_4_shadowing();

    println!("\n\n---- //PATTER MATCH// ---- \n\n");
}

//* Example

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter
}

/*
    In this case the match statement returns a value
    dependending on the matched enum variant
*/
fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter => 25
    }
}

fn match_to_return_value() {
    let coin_penny: Coin = Coin::Penny;
    let coin_nickel: Coin = Coin::Nickel;
    let coin_dime: Coin = Coin::Dime;
    let coin_quarter: Coin = Coin::Quarter;

    let penny_value: u8 = value_in_cents(coin_penny);
    let nickel_value: u8 = value_in_cents(coin_nickel);
    let dime_value: u8 = value_in_cents(coin_dime);
    let quarter_value: u8 = value_in_cents(coin_quarter);

    println!("penny value: {}",penny_value);
    println!("Nickel value: {}",nickel_value);
    println!("Dime value: {}",dime_value);
    println!("Quarter value: {}",quarter_value);
}

/*
    In an Option<T> enum we know we have either 
        - Some(T)
        - None
    But this next pattern is applicable to any other enum.

    Basically it's a less verbose way to say:
        - if it matches THIS specific case good
        - else for EVERY other case do this instead
*/
fn matching_default_case() {
    let config_max: Option<u8> = Some(3u8);

    match config_max {
        Some(max) => println!("\nMax configured to be: {} - VERBOSE", max),
        _ => () //? this syntax says "for every other case return a unit type"
    }

    // But we can do it in a less verbose way using 'if let'
    if let Some(max) = config_max {
        println!("Max configured to be: {} - NON VERBOSE", max)
    }

    // Same thing with other enums
    let coin: Coin = Coin::Dime;
    match coin {
        Coin::Dime => println!("It's a DIME! - VERBOSE"),
        _ => println!("Not a DIME.")
    }

    // insead of match every possible case, or using the '_' operator we use
    // if let
    if let Coin::Dime = coin {
        println!("It's a DIME! - NON VERBOSE")
    }
}

/*
    In a match statement I can use logic operators
*/
fn match_with_logic_operators(){

    let coin: Coin = Coin::Dime;

    match coin {
        Coin::Penny | Coin::Nickel => println!("\n Value <= 10"),
        Coin::Dime | Coin::Quarter => println!("\n Value >= 10")
    }

}

/*
    Match statements can be used in assingment
*/
fn assignement_with_match(){
    let coin: Coin = Coin::Quarter;
    let coin_value = match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter => 25
    };

    println!("\nCoin value is: {}", coin_value)
}

enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

/*
    Using match to get the data an enum variant holds.
*/

fn accessing_enum_data_via_match_statement() {
    println!("\n");
    let msgs: [Message; 4] = [
        Message::Quit,
        Message::Move{x:1, y:3},
        Message::ChangeColor(255,255,0),
        Message::Write("Ciao".to_string())
    ];

    for msg in msgs {
        show_message(msg)
    }

    println!("Success!");
} 

fn show_message(msg: Message) {
    match msg {
        Message::Move{x:a, y:b} => { // match  Message::Move
            assert_eq!(a, 1);
            assert_eq!(b, 3);
            println!("Message type move, values: x: {}, y:{}", a, b);
        },
        Message::ChangeColor(r, g, b) => {
            assert_eq!(g, 255);
            assert_eq!(b, 0);
            println!("Message type change color, values: r:{} g:{} b:{} ", r, g, b);
        },
        Message::Write(s) => {
            println!("Messagge type Write, value: {}", s)
        }
        _ => println!("no data in these variants")
    }
}

/*
    matches! looks like match, but can do something different.

    Returns whether the given expression matches any of the given patterns.

    Like in a match expression, the pattern can be optionally followed by if and a 
    guard expression that has access to names bound by the pattern.
*/
fn matches_macro() {
    let alphabets: [char; 7] = ['a', 'E', 'Z', '0', 'x', '9' , 'Y'];

    for ab in alphabets {
        assert!(matches!(ab, 'A'..='Z' | 'a'..='z' | '0'..='9'));
    }

    println!("\nSuccess! - matches! macro");
}


enum MyEnum {
    Foo,
    Bar
}

fn exercise_1() {
    println!("\nEXERCISE 1");

    let mut count: i32 = 0;

    let v: Vec<MyEnum> = vec![MyEnum::Foo,MyEnum::Bar,MyEnum::Foo];
    for e in v {
        if let MyEnum::Foo = e {
            count += 1;
        }
    }

    assert_eq!(count, 2);

    println!("Success!");
}

enum Foo {
    Bar(u8),
    _FooBar
}

fn exercise_2() {
    println!("\nEXERCISE 2");

    let a: Foo = Foo::Bar(1);

    if let Foo::Bar(i) = a {
        println!("foobar holds the value: {}", i);

        println!("Success!");
    }
}


enum Foo2 {
    Bar,
    Baz,
    Qux(u32)
}

fn exercise_3() {
    println!("\nEXERCISE 3");

    let a: Foo2 = Foo2::Qux(10);
    let b: Foo2 = Foo2::Bar;
    let c: Foo2 = Foo2::Baz;

    // Remove the codes below, using `match` instead 
    // if let Foo2::Bar = a {
    //     println!("match foo::bar")
    // } else if let Foo2::Baz = a {
    //     println!("match foo::baz")
    // } else {
    //     println!("match others")
    // }

    match a {
        Foo2::Bar => {
            println!("a match foo::bar")
        }
        Foo2::Baz => {
            println!("a match foo::baz")
        },
        _ => {
            println!("a match others")
        },
    }
    match b {
        Foo2::Bar => {
            println!("b match foo::bar")
        }
        Foo2::Baz => {
            println!("b match foo::baz")
        },
        _ => {
            println!("b match others")
        },
    }
    match c {
        Foo2::Bar => {
            println!("c match foo::bar")
        }
        Foo2::Baz => {
            println!("c match foo::baz")
        },
        _ => {
            println!("c match others")
        },
    }
}

/*
    Shadowing when destructuring
    ? Non ho ancora capito bene questa cosa
*/
fn exercise_4_shadowing() {
    let age: Option<i32> = Some(30);
    
    if let Some(age) = age { // Creating a new variable with the same name as previous `age`
       assert_eq!(age, 30);
    } // The new variable `age` goes out of scope here

    //? Which is basically the same of doing:
    if let Some(pippo) = age {
       assert_eq!(pippo, 30);
    } // The new variable `pippo` goes out of scope here
    //? but here we use a different name
    

    //? stessa cosa con match, when destructuring posso usare una variabile
    //? con lo stesso nome della variabile da matchare 
    match age {
        // Match can also introduce a new shadowed variable
        Some(age) =>  println!("age is a new variable, it's value is {}",age),
        _ => ()
    }
}
