fn main() {
    //Integers
    /*let a = 98_222; //Decimal
    let b = 0xff; //Hex
    let c = 0o77; // Octal
    let d = 0b1111_0000; //Binary
    let e = b'A'; //Byte (u8 only)

    let f: u8 = 255;

    //Floating-point numbers
    let f = 2.0;
    let g:f32 = 3.0;

    //Compound Types

    //Tuple - Array that consists of values with different types
    let tup = ("Hello World",100_000);*/

    let sum = my_function(5, 10);
    println!("The sum is {sum}");

    control_loop_with_return();

    for_loop();
}

fn my_function(x:i32,y:i32) -> i32{
    println!("Hello {x} and {y} thrown");
    
    return x + y;
}

fn control_loop_with_return(){
    let mut counter = 0;

    let result = loop{
        counter+=1;

        if counter == 10{
            break counter;
        }
    };

    println!("result is {result}");
}

fn for_loop(){

    let a = [10,20,30,40,50];

    for element in a.iter(){
        println!("{}",element);
    }
}
