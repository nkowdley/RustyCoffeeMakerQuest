
use std::io;
use std::io::prelude::*;
mod player;
mod room;

#[allow(dead_code)]
fn main() {
	/*Print the welcome message*/
	println!("Coffee Maker Quest v2.0: The Rusty Coffee Maker");
	println!("Instructions for Coffee Maker Quest - ");
	println!("You are a brave student trying to study for the final exam in Bill Laboon's Software Testing Class, but you need caffeine.");
	println!("The goal of the game is to collect sugar, coffee, and cream so that you can study.");
	/*initialize the array of rooms*/
	let house : [room::Room; 6] = [ room::Room::new(0), room::Room::new(1), room::Room::new(2), room::Room::new(3), room::Room::new(4), room::Room::new(5) ];
	/*intialize a player*/
	let mut p = player::Player::new();
	/*set a boolean to stop the game*/
	let mut run : bool = true;

	while (run)
	{
		let mut input:String = get_user_input();
		//if prompt_user_input:
		while !validate_user_input(input){
			input = get_user_input();
		}


		//do things
	}

}

fn get_user_input() -> String {
	println!("Please enter a command!");
	let mut input = String::new();
	try!(io::stdin().read_line(&mut input));
	let mut input_trimmed = input.trim();
    Ok(input_trimmed)
}

fn validate_user_input(input:String) -> bool {
	false
}

#[test] //Tests each possible input.
fn validate_user_input_test1() {
	let test_string = "n".to_string();
	let validate_result = validate_user_input(test_string);
	assert!(true, validate_result);
}

#[test]//Tests S as input
fn validate_user_input_test2(){
	let test_string = "s".to_string();
	let validate_result = validate_user_input(test_string);
	assert!(true, validate_result);

}

#[test] //Tests L for input
fn validate_user_input_test3(){
	let test_string = "l".to_string();
	let validate_result = validate_user_input(test_string);
	assert!(true, validate_result);
}

#[test] //Tests I for input
fn validate_user_input_test4(){
	let test_string = "i".to_string();
	let validate_result = validate_user_input(test_string);
	assert!(true, validate_result);
}

#[test] //Tests H for input
fn validate_user_input_test5(){
	let test_string = "h".to_string();
	let validate_result = validate_user_input(test_string);
	assert!(true, validate_result);
}

#[test] //Tests for D for input
fn validate_user_input_test6(){
	let test_string = "d".to_string();
	let validate_result = validate_user_input(test_string);
	assert!(true, validate_result);
}
#[test] //Tests for a string of numbers
fn validate_user_input_test7(){
	let test_string = "01234567891011121314151617181920".to_string();
	let validate_result = validate_user_input(test_string);
	assert!(false, validate_result);	
}
#[test] //Tests for a string of mixed cases/characters and also will serve as a test for input other than standard commands.
fn validate_user_input_test8(){
	let test_string = "$HelloWorld!@".to_string();
	let validate_result = validate_user_input(test_string);
	assert!(false, validate_result);	
}