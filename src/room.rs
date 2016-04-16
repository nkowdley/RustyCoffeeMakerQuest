/*
* Authors: Neel Kowdley <nkowdley@gmail.com>, David Sweeney <dms163@pitt.edu>
* Class: CS1632
* Project: Deliverable 6: CoffeeMakerQuest in Rust
* File: room.rs
*/
/*initialize the room struct*/
struct Room {
	has_north_door: bool ,
	has_south_door: bool,
	description: String,
	adj:String
}
/*Room methods*/
#[allow(dead_code)]
impl Room {
	/*Constructor*/
	pub fn new(room_num: i32) -> Room {
		if room_num==0 {
			Room {
				has_north_door: true,
				has_south_door: false,
				description: "".to_string(),
				adj:"".to_string()
			}
		}
		else  {
			Room {
				has_north_door: true,
				has_south_door: false,
				description: "".to_string(),
				adj:"".to_string()
			}
		}
	}

}
/*Test Cases*/
/*Verify that the starting room(index 0) only has a north door*/
#[test]
fn room_0_north_door() {
	/*Allocate a new room*/
	let r = Room::new(0);
	assert!(r.has_north_door,true);
}
