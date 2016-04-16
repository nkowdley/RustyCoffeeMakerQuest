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
				description: "a statue of Bill Laboon".to_string(),
				adj:"".to_string()
			}
		}
		else if room_num==1 {
			Room {
				has_north_door: true,
				has_south_door: true,
				description: "Amazon's best-seller, \"A Friendly Introduction to Software Testing\" by THE Bill Laboon".to_string(),
				adj:"".to_string()
			}
		}
		else if room_num==2 {
			Room {
				has_north_door: true,
				has_south_door: true,
				description:  "an autographed photo of Bill Laboon".to_string(),
				adj:"".to_string()
			}
		}
		else if room_num==3 {
			Room {
				has_north_door: true,
				has_south_door: true,
				description:"\"Hackin' Fellow\" on repeat 'cause it's such an amazing song".to_string(),
				adj:"".to_string()
			}
		}
		else if room_num==4 {
			Room {
				has_north_door: true,
				has_south_door: true,
				description: "a broken metronome that is constantly on high".to_string(),
				adj:"".to_string()
			}
		}
		else  {
			Room {
				has_north_door: false,
				has_south_door: true,
				description: "A number of Cats from the billion dollar enterprise: Rent A Cat".to_string(),
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
	assert!(r.has_north_door);
}

#[test]
fn room_1_north_door() {
	/*Allocate a new room*/
	let r = Room::new(1);
	assert!(r.has_north_door);
}
#[test]
fn room_2_north_door() {
	/*Allocate a new room*/
	let r = Room::new(2);
	assert!(r.has_north_door);
}
#[test]
fn room_3_north_door() {
	/*Allocate a new room*/
	let r = Room::new(3);
	assert!(r.has_north_door);
}
#[test]
fn room_4_north_door() {
	/*Allocate a new room*/
	let r = Room::new(4);
	assert!(r.has_north_door);
}
#[test]
fn room_5_north_door() {
	/*Allocate a new room*/
	let r = Room::new(5);
	assert_eq!(r.has_north_door,false);
}
/*Test whether each room has the correct status for the south door*/
#[test]
fn room_0_south_door() {
	/*Allocate a new room*/
	let r = Room::new(0);
	assert_eq!(r.has_south_door,false);
}
#[test]
fn room_1_south_door() {
	/*Allocate a new room*/
	let r = Room::new(1);
	assert!(r.has_south_door);
}
#[test]
fn room_2_south_door() {
	/*Allocate a new room*/
	let r = Room::new(2);
	assert!(r.has_south_door);
}
#[test]
fn room_3_south_door() {
	/*Allocate a new room*/
	let r = Room::new(3);
	assert!(r.has_south_door);
}
#[test]
fn room_4_south_door() {
	/*Allocate a new room*/
	let r = Room::new(4);
	assert!(r.has_south_door);
}
#[test]
fn room_5_south_door() {
	/*Allocate a new room*/
	let r = Room::new(5);
	assert!(r.has_south_door);
}
/*Test whether each room has the correct descriptionr*/
#[test]
fn room_0_description() {
	/*Allocate a new room*/
	let r = Room::new(0);
	assert_eq!(r.description,"a statue of Bill Laboon");
}
#[test]
fn room_1_description() {
	/*Allocate a new room*/
	let r = Room::new(1);
	assert_eq!(r.description,"Amazon's best-seller, \"A Friendly Introduction to Software Testing\" by THE Bill Laboon");
}
#[test]
fn room_2_description() {
	/*Allocate a new room*/
	let r = Room::new(2);
	assert_eq!(r.description,"an autographed photo of Bill Laboon");
}
#[test]
fn room_3_description() {
	/*Allocate a new room*/
	let r = Room::new(3);
	assert_eq!(r.description,"\"Hackin' Fellow\" on repeat 'cause it's such an amazing song");
}
#[test]
fn room_4_description() {
	/*Allocate a new room*/
	let r = Room::new(4);
	assert_eq!(r.description,"a broken metronome that is constantly on high");
}
#[test]
fn room_5_description() {
	/*Allocate a new room*/
	let r = Room::new(5);
	assert_eq!(r.description,"A number of Cats from the billion dollar enterprise: Rent A Cat");
}
