
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
			println!("Invalid Command!");
			input = get_user_input();
		}

		//do things

	}

}

fn get_user_input() -> String {
	println!("Please enter a command!");
	let mut input = String::new();
    let stdin = io::stdin();
    stdin.lock().read_line(&mut input).unwrap();
    input
}

fn validate_user_input(input:String) -> bool {
	let mut upper_input = String::new();
	let N = "N".to_string();
	let S = "S".to_string();
	let L = "L".to_string();
	let I = "I".to_string();
	let H = "H".to_string();
	let D = "D".to_string();
	upper_input = input.to_uppercase();
	upper_input = upper_input.to_string();
	if upper_input == N {
		return true;
	}
	else if upper_input == S {
		return true;
	}
	else if upper_input == L {
		return true;
	}
	else if upper_input == I {
		return true;
	}
	else if upper_input == H {
		return true;
	}
	else if upper_input == D {
		return true;
	}
	else{
		return false;
	}
}
//End main

/*
Tests Below
Order of tests goes as follows:
	1.) validate_input_tests
	2.)

*/

// 1.) Tests each valid possible input for validate_user_input and tests potential edge cases.

#[test] //Tests lowercase n as input.
fn validate_user_input_test1() {
	let test_string = "n".to_string();
	let validate_result:bool = validate_user_input(test_string);
	assert!(validate_result);
}

#[test]//Tests lowercase s as input
fn validate_user_input_test2(){
	let test_string = "s".to_string();
	let validate_result = validate_user_input(test_string);
	assert!(validate_result);

}

#[test] //Tests lowercase l as input
fn validate_user_input_test3(){
	let test_string = "l".to_string();
	let validate_result = validate_user_input(test_string);
	assert!(validate_result);
}

#[test] //Tests lowercase i as input
fn validate_user_input_test4(){
	let test_string = "i".to_string();
	let validate_result = validate_user_input(test_string);
	assert!(validate_result);
}

#[test] //Tests H as input
fn validate_user_input_test5(){
	let test_string = "h".to_string();
	let validate_result = validate_user_input(test_string);
	assert!(validate_result);
}

#[test] //Tests for lowercase d as input
fn validate_user_input_test6(){
	let test_string = "d".to_string();
	let validate_result = validate_user_input(test_string);
	assert!(validate_result);
}
#[test] //Tests for a string of numbers
fn validate_user_input_test7(){
	let test_string = "01234567891011121314151617181920".to_string();
	let validate_result = validate_user_input(test_string);
	if validate_result == false{
		assert!(true);
	}else{
		assert!(validate_result);
	}	
}
#[test] //Tests for a string of mixed cases/characters and also will serve as a test for input other than standard commands.
fn validate_user_input_test8(){
	let test_string = "$HelloWorld!@".to_string();
	let validate_result = validate_user_input(test_string);
	if validate_result == false{
		assert!(true);
	}else{
		assert!(validate_result);
	}	
}
#[test]//Tests for a valid command with an invalid character.
fn validate_user_input_test9(){
	let test_string = "N@".to_string();
	let validate_result = validate_user_input(test_string);
	assert!(false);	
}
#[test]//Tests for empty string being passed in as input.
fn validate_user_input_test10(){
	let test_string = " ".to_string();
	let validate_result = validate_user_input(test_string);
	assert!(false);	
}
#[test]//Tests for the ascii value of a valid input.
fn validate_user_input_test11(){
	let test_string = "78".to_string();
	let validate_result = validate_user_input(test_string);
	assert!(false);	
}
#[test]//Tests for a string of pure punctuation. 
fn validate_user_input_test12(){
	let test_string = " !.,"";?': ".to_string();
	let validate_result = validate_user_input(test_string);
	assert!(false);	
}
#[test]//Tests for uppercase N
fn validate_user_input_test13(){
	let test_string = "N".to_string();
	let validate_result = validate_user_input(test_string);
	assert!(false);			
}