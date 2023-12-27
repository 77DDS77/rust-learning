

/*
    STATEMENT: 
        Instruction that perform some action but
        DO NOT produce a value.
        EX: assegnazione di una variabile:
            let x = 2; => STATEMENT

    EXPRESSION:
        Evaluate to a resultant value
*/
pub fn _statement_and_expressions(){
    println!("\n\n---- STATEMENT AND EXPRESSIONS ---- \n\n");

    expression();
    println!("Omettendo putno e virgola ritorna {}", omitting_punto_e_virgola());
    exercise_1();
    exercise_2();
    exercise_3();

    println!("\n\n---- //STATEMENT AND EXPRESSIONS// ---- \n\n");
}

fn expression() -> u32 {
    let x:u32 = 5u32;

    let y: u32 = { //* start of the expression
        let x_squared: u32 = x * x;
        let x_cube: u32 = x_squared * x;

        //* This expression will be assigned to `y`
        //* omettere il ; equivale a => return x_cube + x_squared + x
        x_cube + x_squared + x
    };

    y
}

fn omitting_punto_e_virgola() -> u8 {
    2_u8 + 5_u8
}


fn exercise_1() {
    let v = {
        let mut x = 1;
        x += 2;
        x
    };
 
    assert_eq!(v, 3);
 
    println!("Ex 1 - Success!");
}

fn exercise_2(){
    // let v = (let x = 3);
    let v: i32 = {
        let x: i32 =3;
        x
    };

    assert!(v == 3);

    println!("Ex 2 - Success!");
}

fn exercise_3() {
    let s: i32 = sum(1 , 2);
    assert_eq!(s, 3);

    println!("Ex 3 - Success!");
}

fn sum(x: i32, y: i32) -> i32 {
    x + y
}
