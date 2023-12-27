
pub fn _numbers_various(){
    println!("\n\n---- NUMBERS VARIOUS ---- \n\n");

    range();
    range_and_for_loops();
    bitwise_operation();

    println!("\n\n---- //NUMBERS VARIOUS// ---- \n\n");
}

/*
    RANGE:
        - inclusive 
        - non inclusive
*/
fn range(){
    let _range_non_inclusive = -3..2; //-3, -2, -1, 0 , 1
    let _range_inclusive = -3..=2; //-3, -2, -1, 0 , 1, 2
}

/*
    FOR LOOPS:
        - basic declaration: for i in -3..2.
        With this declaration it will stop at i < 2 
*/
fn range_and_for_loops(){

    let mut sum: i32 = 0;

    for i in -3..2 {
        sum += i;
    };

    println!("sum: {}", sum)

}

/*
    Operazioni bitwise:

        * COMPARAZIONE BITWISE
        - sono operazioni che separano, ad esempio, un numbero in singoli bit e 
        comparano i singoli bit invece del numero intero.

        - DICHIARAZIONE:
            - x && y oppure x || y => predicato "classico"
            - x & y oppure x | y => operazione bitwise

        - BREAKDOWN FUNZIONAMENTO:

            0b0011u32 & 0b0101u32 => rappresentazione binaria di 0011 e 0101
            separa in => 0 0 1 1 e 0 1 0 1 
            poi compara partendo da dx
            1 & 1 -> true  => 1
            1 & 0 -> false => 0
            0 & 1 -> false => 0
            0 & 0 -> false => 0

            output => 0001
        
        * BITWISE SHIFT
        Letteralmente sposta di n posizioni i bit di un numero x

        0000 0001 << 5 => sposta di 5 posizioni verso sx il bit
        result => 0010 0000 => 32

*/
fn bitwise_operation() {
    println!("0011 AND 0101 is {:04b}", 0b0011u32 & 0b0101u32);
    println!("0011 OR 0101 is {:04b}", 0b0011u32 | 0b0101u32);
    println!("0011 XOR 0101 is {:04b}", 0b0011u32 ^ 0b0101u32);
    println!("1 << 5 is {}", 0b0001u32 << 5);
    println!("0x80 >> 2 is 0x{:x}", 0x80u32 >> 2);
}