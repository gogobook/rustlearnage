use std::io;
extern crate rand;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    println!("Guess the number!");

//old:    let secret_number = (rand::random::<u32>() % 100) + 1; // secret_number: i32
    //alternate way to type hint from above ^ let x: u32 = rand::random();
    let secret_number = rand::thread_rng().gen_range(1, 101);

//    println!("The secret number is: {}", secret_number);

    assert_eq!(Ordering::Less, 1.cmp(&2));

    loop {
        println!("Please input your guess.");
        let mut guess=String::new();
        io::stdin().read_line(&mut guess)
            .ok()
            .expect("Failed to read line");

        print!("!");
        print!("{}",guess);//this does contain that eol '\n', unless you Ctrl+D twice(in Linux) instead of Enter! (in Windows Ctrl+Z might be it?)
        print!("!");
//        let guess: u8 = guess.trim().parse().expect("not a number!");
        if guess.is_empty() {//ctrl+d once
            println!("Ok, we're out!");
            break;
        }

        let guess: u8 = match guess.trim().parse() {
            Ok(num) => num,
            Err(e) => {
                println!("error: {}",e);
                continue
            },
        };
        //    let input_num = "5".parse::<u32>(); // input_num: Option<u32>
        //    let input_num: Option<u32> = "5".parse(); // input_num: Option<u32>
//old:        let input_num: Option<u32> = input.trim().parse();
//        let input_num: Result<u32,_> = input.trim().parse();
//        let Result<num:u32,_> = input.trim().parse();
/*        let num = match input_num {
            Some(num) => num,
            None => {
                println!("Please input a number!");
                continue;
            }
        };*/


        println!("You guessed: {}", guess);

//        match cmp(guess, secret_number) {
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                assert_eq!(guess.cmp(&secret_number), secret_number.cmp(&guess));
                //return;
                break;
            },
        }
    }//loop
    println!("normal exit.");
}

/*old
fn cmp(a: u32, b: u32) -> Ordering {
    if a < b { Ordering::Less }
    else if a > b { Ordering::Greater }
    else { Ordering::Equal }
}*/
