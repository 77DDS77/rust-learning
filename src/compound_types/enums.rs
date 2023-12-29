
/*
    * ENUMS:
        - way of defining a type with only one possible set of values
        - we can only access one variant of an enum at a time
        - can hold additional informations using tuples
        - especially useful when used in ```match``` statements
*/

pub fn _enums(){

    println!("\n\n---- ENUMS ---- \n\n");
    
    use_enum();
    testing_enums();
    exercise_1();
    matching_v1();
    matching_v2();
    enum_array();
    enum_option();

    println!("\n\n---- //ENUMS// ---- \n\n");

}

// Enum declaration - an enum holds variants
#[derive(Debug)] // to print the enum
enum IpAdrress {
    V4(String), // variant 1
    V6(String) // variant 2
}

fn use_enum(){

    let home: IpAdrress = IpAdrress::V4(String::from("127.0.0.0"));

    let loopback: IpAdrress = IpAdrress::V6(String::from("::1"));

    println!("IpV4: {:?} | IpV6: {:?}", home, loopback);
}

#[derive(Debug)]
enum TestEnum {
    Option1 = 1,
    Option2 = 7,
    Option3     // se non specifico prende l'ultimo specificato + 1
}

fn testing_enums(){

    let t1: TestEnum = TestEnum::Option1;
    let t2: u8 = TestEnum::Option2 as u8; // access them as integer to store the value
    let t3: u8 = TestEnum::Option3 as u8;

    println!("\nTESTING ENUMS:");
    println!("\nt1: {:?} | t2: {:?} | t3: {:?}", t1,t2, t3); // t1: Option1 | t2: 7 | t3: 8

}

#[derive(Debug)]
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

fn exercise_1() {
    let msg1: Message = Message::Move{ x: 1, y: 2}; // Instantiating with x = 1, y = 2 
    let msg2: Message = Message::Write(String::from("Hello world!")); // Instantiating with "hello, world!"
    let msg3: Message = Message::Quit;
    let msg4: Message = Message::ChangeColor(123, 221, 123);


    println!("\n EXCERCISE 1");

    // matching
    match &msg1 {
        Message::Move { x, y } => {
            println!("msg1: x = {} y = {}", x, y);
        }
        Message::Write(s) => {
            println!("{}", s)
        }
        _ => {
            println!("msg1 is not of type Move");
        }
    }

    println!("msg1:{:?} msg2:{:?} msg3:{:?} msg4:{:?}", msg1, msg2, msg3, msg4);
}

fn matching_v1() {
    println!("\n Matching v1");
    // let msg: Message = Message::Move{x: 1, y: 1};
    let msg: Message = Message::Write(String::from("Hej"));

    if let Message::Move{ x: a, y: b} = msg {
        assert_eq!(a, b);
        println!("\nmsg is of type ::Move")
    } else if let Message::Write(s) = msg {
        println!("\n{}",s);
        println!("msg is of type ::Write");
    } else {
        panic!("\nNEVER LET THIS RUN!");
    }

    println!("Success!\n"); 

}

fn matching_v2(){
    println!("\n Matching v2");
    // let msg: Message = Message::Move{x: 1, y: 1};
    // let msg: Message = Message::Write(String::from("Hej"));
    // let msg: Message = Message::Quit;
    let msg: Message = Message::ChangeColor(123, 64, 12);

    match &msg {
        Message::Move { x, y } => {
            println!("\nmsg: x = {} y = {}", x, y);
            println!("msg is of type ::Move");
        }
        Message::Write(s) => {
            println!("\n{}", s);
            println!("msg is of type ::Write\n");
        }
        Message::Quit => {
            println!("\nmsg is of type ::Quit\n");
        }
        Message::ChangeColor(r, g, b) => {
            println!("\nmsg: r = {} g = {} b = {}", r, g, b);
            println!("msg is of type ::Quit\n");
        }
    };

}

fn enum_array() {
    let msgs: [Message; 3] = [
        Message::Quit,
        Message::Move{x:1, y:3},
        Message::ChangeColor(255,255,0)
    ];

    for msg in msgs {
        show_message(msg)
    }
} 

fn show_message(msg: Message) {
    println!("{:?}", msg);
}


/*
    * Enum Option<T> :
        - Option<t> is an enum that represent a value that 
            may or may not be present
        - known in other languages as "null" refering to 
            the absence of a value
        - used to handle cases where a function or method might 
            fail to return a value

        In the std library is defined like this:

            enum  Option<T> {
                None,
                Some(T)
            }

        
*/

fn enum_option() {
    println!("\n\n   ---- Option<T> ---- \n\n");
    
    let five: Option<i32> = Some(5);
    let six: Option<i32> = plus_one(five);
    let none: Option<i32> = plus_one(None);
    
    println!("six: {:?} | None: {:?}", six, none);

    if let Some(n) = six {
        println!("accessing the value in Some({}) = {}", n, n);
    }

    println!("\n\n   ---- //Option<T>// ---- \n\n");
} 

fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1),
    }
}
