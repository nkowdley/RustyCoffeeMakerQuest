/*
* Authors: Neel Kowdley <nkowdley@gmail.com>, David Sweeney <dms163@pitt.edu>
* Class: CS1632
* Project: Deliverable 6: CoffeeMakerQuest in Rust
* File: player.rs
*/
/*Initialize the Player Struct*/
struct Player {
	has_cream: bool,
	has_coffee: bool,
	has_sugar:bool
}
/*implement the player*/
#[allow(dead_code)]
impl Player {
	/*Constructor*/
   fn new() -> Player {
	Player {
		has_cream : false,
		has_coffee : false,
		has_sugar : false
	}
	}
}
/*Test Cases*/
/*Initial Test Case to prove that Cargo Test works*/
#[test]
fn it_works() {
    assert_eq!(4, 4);
}
/*Test that the Constructor does everything it needs to*/
#[test]
#[should_panic(expected = "assertion failed")] /*Mark that this test case should fail*/
fn test_constructor() {
	let p = Player::new();
	/*Verify the player does not have cream, coffee or sugar*/
	assert!(p.has_cream);
	assert!(p.has_coffee);
	assert!(p.has_sugar);
}
// fn can_win() {
// 	p.get_cream();
// 	p.get_coffee();
// 	p.get_sugar();
// 	assert!(p.can_win());
// }
