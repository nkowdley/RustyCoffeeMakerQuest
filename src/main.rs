mod player;
mod room;

fn main() {
	/*Print the welcome message*/
	println!("Coffee Maker Quest v2.0: The Rusty Coffee Maker");
	println!("Instructions for Coffee Maker Quest - ");
	println!("You are a brave student trying to study for the final exam in Bill Laboon's Software Testing Class, but you need caffeine.");
	println!("The goal of the game is to collect sugar, coffee, and cream so that you can study.");
	/*initialize the array of rooms*/
	let house : [room::Room; 6] = [ room::Room::new(0), room::Room::new(1), room::Room::new(2), room::Room::new(3), room::Room::new(4), room::Room::new(5) ];

}
