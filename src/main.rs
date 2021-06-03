
mod num_manage;
fn main(){
 let num_list:[i32;10] = [1,2,3,4,5,6,7,8,9,10];
 const NUMBER_OF_TURNS:i32 = 5;
 let mut questions_wrong:i32 = 0;
while questions_wrong < NUMBER_OF_TURNS {
    println!("please enter a number, 1 through 10");
    let mut hidden_number = num_manage::select_number(num_list);
    let mut player_answer = num_manage::user_input();
 }
 

}