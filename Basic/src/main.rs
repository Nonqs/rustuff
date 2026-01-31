use std::io;

fn main() {
    println!("Hello, world!");

    //scope

    let x = 4;
    println!("x is: {}", x); //Result = 4

    {
        let x = x - 2;
        println!("x is: {}", x); //Result = 2
    }

    let x = x + 1;
    println!("x is: {}", x); // Result = 4

    //Get inputs

    let mut inpt = String::new();

    io::stdin().read_line(&mut inpt).expect("failed to read line");
    println!("{}", inpt);

    //Call a fn

    test_one(); 

    let number ={
        let x = 3;
        x + 1;
    };
}

fn test_one(){
    println!("Test");
}
