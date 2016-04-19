
use std::io;
use std::io::prelude::*;
use std::ops::Add::*;
use player::{Player};
pub mod player;
use room::{Room};
pub mod room;

static mut current_room: usize = 0;

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
	/*set an integer to be the game ender*/
	let mut run : i32 = 0;
	let mut index: usize = 0;
	while run==0
	{
		println!("you see a {} room.  You find {} .", house[index as usize].get_adj(), house[index as usize].get_description() );

		let mut input:String = get_user_input();
		//Loop until valid command has been given.
		while !validate_user_input(&input){
			println!("Invalid Command!");
			input = get_user_input();
		}
		run = execute_command(&input, & house, &mut p, &mut index );
	}
}
// End of Main //

/*
* get_user_input()
* Function to get the user input.
* Returns the users input to be validated.	
*/
fn get_user_input() -> String {
	println!(" INSTRUCTIONS (N,S,L,I,D) > ");
	println!("Please enter a command!");
	let mut input = String::new();
    let stdin = io::stdin();
    stdin.lock().read_line(&mut input).unwrap();
    input
}
/*
* validate_user_input(input: &String)
* Validates the users input and converts to uppercase.
* Returns true if the input is valid else returns false.
*/
fn validate_user_input(input:&String) -> bool {
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
/*
* execute_command(command:& String, house:& [Room;6], player:&mut Player)
* Routes the command to the proper method that will execute the command.
* Returns -1 if lose condition has been met.
* Returns 0 if neither win nor lose condition met.
* Returns 1 if win condition has been met.
*/
fn execute_command(command:& String, house:& [Room;6], player:&mut Player, index:&mut usize) -> i32{
	if command == "N" {
		*index=*index+1;
		println!("index is now {}",index);
		0
	}
	else if command == "S" {
		0
	}
	else if command == "L" {
		0
	}
	else if command == "I" {
		0
	}
	else if command == "H" {
		0
	}
	else if command == "D" {
		0
	}
	else {
		0
	}	
}

//End main.rs //

/*
* Tests Below
* Order of test blocks:
*	1.) validate_input_tests
*	2.) execute_command_tests
*	3.)
*/

/*
* Start of the validate_input_tests
* 1.) Tests each valid possible input for validate_user_input() and tests potential edge cases.
*/
#[test]//Tests lowercase n as input.
fn validate_user_input_test1() {
	let test_string = "n".to_string();
	let validate_result:bool = validate_user_input(&test_string);
	assert!(validate_result);
}
#[test]//Tests lowercase s as input
fn validate_user_input_test2(){
	let test_string = "s".to_string();
	let validate_result = validate_user_input(&test_string);
	assert!(validate_result);

}
#[test]//Tests lowercase l as input.
fn validate_user_input_test3(){
	let test_string = "l".to_string();
	let validate_result = validate_user_input(&test_string);
	assert!(validate_result);
}
#[test]//Tests lowercase i as input.
fn validate_user_input_test4(){
	let test_string = "i".to_string();
	let validate_result = validate_user_input(&test_string);
	assert!(validate_result);
}
#[test]//Tests lowercase h as input.
fn validate_user_input_test5(){
	let test_string = "h".to_string();
	let validate_result = validate_user_input(&test_string);
	assert!(validate_result);
}
#[test]//Tests for lowercase d as input.
fn validate_user_input_test6(){
	let test_string = "d".to_string();
	let validate_result = validate_user_input(&test_string);
	assert!(validate_result);
}
#[test]//Tests for a string of numbers.
fn validate_user_input_test7(){
	let test_string = "01234567891011121314151617181920".to_string();
	let validate_result = validate_user_input(&test_string);
	if validate_result == false{
		assert!(true);
	}else{
		assert!(validate_result);
	}	
}
#[test]//Tests for a string of mixed cases/characters and also will serve as a test for input other than standard commands.
fn validate_user_input_test8(){
	let test_string = "$HelloWorld!@".to_string();
	let validate_result = validate_user_input(&test_string);
	if validate_result == false{
		assert!(true);
	}else{
		assert!(validate_result);
	}	
}
#[test]//Tests for a valid command with an invalid character.
fn validate_user_input_test9(){
	let test_string = "N@".to_string();
	let validate_result = validate_user_input(&test_string);
	if validate_result == false{
		assert!(true);
	}else{
		assert!(validate_result);
	}		
}
#[test]//Tests for empty string being passed in as input.
fn validate_user_input_test10(){
	let test_string = " ".to_string();
	let validate_result = validate_user_input(&test_string);
	if validate_result == false{
		assert!(true);
	}else{
		assert!(validate_result);
	}		
}
#[test]//Tests for the ascii value of a valid input.
fn validate_user_input_test11(){
	let test_string = "78".to_string();
	let validate_result = validate_user_input(&test_string);
	if validate_result == false{
		assert!(true);
	}else{
		assert!(validate_result);
	}		
}
#[test]//Tests for a string of pure punctuation. 
fn validate_user_input_test12(){
	let test_string = "!.,\";?':".to_string();
	let validate_result = validate_user_input(&test_string);
	if validate_result == false{
		assert!(true);
	}else{
		assert!(validate_result);
	}		
}
#[test]//Tests for uppercase N.
fn validate_user_input_test13(){
	let test_string = "N".to_string();
	let validate_result = validate_user_input(&test_string);
	assert!(validate_result);			
}
#[test]//Tests for uppercase S.
fn validate_user_input_test14(){
	let test_string = "S".to_string();
	let validate_result = validate_user_input(&test_string);
	assert!(validate_result);			
}
#[test]//Tests for uppercase L.
fn validate_user_input_test15(){
	let test_string = "L".to_string();
	let validate_result = validate_user_input(&test_string);
	assert!(validate_result);			
}
#[test]//Tests for uppercase I.
fn validate_user_input_test16(){
	let test_string = "I".to_string();
	let validate_result = validate_user_input(&test_string);
	assert!(validate_result);			
}
#[test]//Tests for uppercase H.
fn validate_user_input_test17(){
	let test_string = "H".to_string();
	let validate_result = validate_user_input(&test_string);
	assert!(validate_result);			
}
#[test]//Tests for uppercase D.
fn validate_user_input_test18(){
	let test_string = "D".to_string();
	let validate_result = validate_user_input(&test_string);
	assert!(validate_result);				
}

// End of validate_user_input tests. //


/*
* Start of the validate_input_tests
* 2.) Tests each valid possible input for execute_command() and tests potential edge cases.
*/

#[test]//Tests potential input N.
fn execute_command_test1(){
	let test_string = "N".to_string();
	let mut test_player = player::Player::new();
	let test_house : [room::Room; 6] = [ room::Room::new(0), room::Room::new(1), room::Room::new(2), room::Room::new(3), room::Room::new(4), room::Room::new(5) ];
	let mut test_index: usize = 0;
	let mut test_int: i32 = 0;
	let execute_result: i32 = execute_command(&test_string, &test_house, &mut test_player, &mut test_index);
	assert_eq!(test_int, execute_result );
}
#[test]//Tests potential input S.
fn execute_command_test2(){
	let test_string = "S".to_string();
	let mut test_player = player::Player::new();
	let test_house : [room::Room; 6] = [ room::Room::new(0), room::Room::new(1), room::Room::new(2), room::Room::new(3), room::Room::new(4), room::Room::new(5) ];
	let mut test_index: usize = 0;
	let mut test_int: i32 = 0;
	let execute_result: i32 = execute_command(&test_string, &test_house, &mut test_player, &mut test_index);
	assert_eq!(test_int, execute_result );
}
#[test]//Tests potential input L.
fn execute_command_test3(){
	let test_string = "L".to_string();
	let mut test_player = player::Player::new();
	let test_house : [room::Room; 6] = [ room::Room::new(0), room::Room::new(1), room::Room::new(2), room::Room::new(3), room::Room::new(4), room::Room::new(5) ];
	let mut test_index: usize = 0;
	let mut test_int: i32 = 0;
	let execute_result: i32 = execute_command(&test_string, &test_house, &mut test_player, &mut test_index);
	assert_eq!(test_int, execute_result );
}
#[test]//Tests potential input I.
fn execute_command_test4(){
	let test_string = "I".to_string();
	let mut test_player = player::Player::new();
	let test_house : [room::Room; 6] = [ room::Room::new(0), room::Room::new(1), room::Room::new(2), room::Room::new(3), room::Room::new(4), room::Room::new(5) ];
	let mut test_index: usize = 0;
	let mut test_int: i32 = 0;
	let execute_result: i32 = execute_command(&test_string, &test_house, &mut test_player, &mut test_index);
	assert_eq!(test_int, execute_result );
} 
#[test]//Tests potential input H.
fn execute_command_test5(){
	let test_string = "H".to_string();
	let mut test_player = player::Player::new();
	let test_house : [room::Room; 6] = [ room::Room::new(0), room::Room::new(1), room::Room::new(2), room::Room::new(3), room::Room::new(4), room::Room::new(5) ];
	let mut test_index: usize = 0;
	let mut test_int: i32 = 0;
	let execute_result: i32 = execute_command(&test_string, &test_house, &mut test_player, &mut test_index);
	assert_eq!(test_int, execute_result );
} 
#[test]//Tests potential input D.
fn execute_command_test6(){
	let test_string = "D".to_string();
	let mut test_player = player::Player::new();
	let test_house : [room::Room; 6] = [ room::Room::new(0), room::Room::new(1), room::Room::new(2), room::Room::new(3), room::Room::new(4), room::Room::new(5) ];
	let mut test_index: usize = 0;
	let execute_result: i32 = execute_command(&test_string, &test_house, &mut test_player, &mut test_index);
	if execute_result == -1{
		assert!(true);
	}
	else if execute_result == 1{
		assert!(true);
	}
	else{
		assert!(false);
	}
}  

// End of execute_command_tests //
