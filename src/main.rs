fn main() {
    //sets while loop to be active or inactive
    let count: bool = false;
}

fn input(){

    println!("please enter in a number");
    // stores the users guesses
    let mut guess_container:[String;5];

    std::io::stdin().read_line(&mut guess_container)
    .expect("failed to enter value");
     //converts every number in the array of strings to a integer value so we can compare them;
    for i in 0..guess_container.len(){
        let guess_container: u32 = guess_container.trim().parse().expect("Please type in a number!");
    }
    
}