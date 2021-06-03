use rand::Rng;
use std::io;
//takes in array as parameter, returns a random number from the list
pub fn select_number(num_list: [i32;10]) -> i32{
let random_index = rand::thread_rng().gen_range(0..num_list.len());
return num_list[random_index];
}


pub fn user_input() -> i32{
    let mut user_answer= String::new();
    io::stdin().read_line( &mut user_answer);
    let user_num_answer:i32 = user_answer.parse().unwrap();
     return user_num_answer;
 
        
  
     
    
}