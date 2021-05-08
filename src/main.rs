fn main() {
    //sets while loop to be active or inactive
   input();
}

fn input(){

    println!("please enter in 5 numbers");
    // stores the users guesses
    let mut guess_container: Vec<i32> = vec![];
    let mut buffer = String::new();
for _i in 0..4{ 
    std::io::stdin().read_line(&mut buffer)
    .expect("failed to enter value");
    
   let buffer: i32 = buffer.parse().expect("Invalid number");
   guess_container.push(buffer);



   }
   
   println!("{:?}",guess_container);
    
}