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
		else if room_num==1 {
			Room {
				has_north_door: true,
				has_south_door: false,
				description: "".to_string(),
				adj:"".to_string()
			}
		}
		else if room_num==2 {
			Room {
				has_north_door: true,
				has_south_door: false,
				description: "".to_string(),
				adj:"".to_string()
			}
		}
		else if room_num==3 {
			Room {
				has_north_door: true,
				has_south_door: false,
				description: "".to_string(),
				adj:"".to_string()
			}
		}
		else if room_num==4 {
			Room {
				has_north_door: true,
				has_south_door: false,
				description: "".to_string(),
				adj:"".to_string()
			}
		}
		else  {
			Room {
				has_north_door: false,
				has_south_door: false,
				description: "".to_string(),
				adj:"".to_string()
			}
		}
	}

}
/*Test Cases*/
/*Verify that all doors have the correct status for the door to the north*/
#[test]
fn room_0_north_door() {
	/*Allocate a new room*/
	let r = Room::new(0);
	assert!(r.has_north_door,true);
}
#[test]
fn room_1_north_door() {
	/*Allocate a new room*/
	let r = Room::new(1);
	assert!(r.has_north_door,true);
}
#[test]
fn room_2_north_door() {
	/*Allocate a new room*/
	let r = Room::new(2);
	assert!(r.has_north_door,true);
}
#[test]
fn room_3_north_door() {
	/*Allocate a new room*/
	let r = Room::new(3);
	assert!(r.has_north_door,true);
}
#[test]
fn room_4_north_door() {
	/*Allocate a new room*/
	let r = Room::new(4);
	assert!(r.has_north_door,true);
}
#[test]
fn room_5_north_door() {
	/*Allocate a new room*/
	let r = Room::new(5);
	assert!(r.has_north_door,false);
}
/*Test whether each room has the correct status for the south door*/
#[test]
fn room_0_south_door() {
	/*Allocate a new room*/
	let r = Room::new(0);
	assert!(r.has_south_door,true);
}
#[test]
fn room_1_south_door() {
	/*Allocate a new room*/
	let r = Room::new(1);
	assert!(r.has_south_door,true);
}
#[test]
fn room_2_south_door() {
	/*Allocate a new room*/
	let r = Room::new(2);
	assert!(r.has_south_door,true);
}
#[test]
fn room_3_south_door() {
	/*Allocate a new room*/
	let r = Room::new(3);
	assert!(r.has_south_door,true);
}
#[test]
fn room_4_south_door() {
	/*Allocate a new room*/
	let r = Room::new(4);
	assert!(r.has_south_door,true);
}
#[test]
fn room_5_south_door() {
	/*Allocate a new room*/
	let r = Room::new(5);
	assert!(r.has_south_door,false);
}
