/*
* Authors: Neel Kowdley <nkowdley@gmail.com>, David Sweeney <dms163@pitt.edu>
* Class: CS1632
* Project: Deliverable 6: CoffeeMakerQuest in Rust
* File: room.rs
*/
/*initialize the room struct*/
pub struct Room {
	has_north_door: bool ,
	has_south_door: bool,
	room_has_coffee: bool,
	room_has_sugar: bool,
	room_has_cream: bool,
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
				room_has_cream: true,
				room_has_coffee: false,
				room_has_sugar: false,
				description: "a statue of Bill Laboon".to_string(),
				adj:"Inspirational".to_string()
			}
		}
		else if room_num==1 {
			Room {
				has_north_door: true,
				has_south_door: true,
				room_has_cream: false,
				room_has_coffee: true,
				room_has_sugar: false,
				description: "Amazon's best-seller, \"A Friendly Introduction to Software Testing\" by THE Bill Laboon".to_string(),
				adj:"Cool-Dude".to_string()
			}
		}
		else if room_num==2 {
			Room {
				has_north_door: true,
				has_south_door: true,
				room_has_cream: false,
				room_has_coffee: false,
				room_has_sugar: false,
				description:  "an autographed photo of Bill Laboon".to_string(),
				adj:"Chili-Pepper".to_string()
			}
		}
		else if room_num==3 {
			Room {
				has_north_door: true,
				has_south_door: true,
				room_has_cream: false,
				room_has_coffee: false,
				room_has_sugar: false,
				description:"\"Hackin' Fellow\" on repeat 'cause it's such an amazing song".to_string(),
				adj:"Smart".to_string()
			}
		}
		else if room_num==4 {
			Room {
				has_north_door: true,
				has_south_door: true,
				room_has_cream: false,
				room_has_coffee: false,
				room_has_sugar: false,
				description: "a broken metronome that is constantly on high".to_string(),
				adj:"Fun".to_string()
			}
		}
		else  {
			Room {
				has_north_door: false,
				has_south_door: true,
				room_has_cream: false,
				room_has_coffee: false,
				room_has_sugar: true,
				description: "A number of Cats from the billion dollar enterprise: Rent A Cat".to_string(),
				adj:"Hilarious".to_string()
			}
		}
	}
	pub fn get_north_door(&self) -> bool {
		self.has_north_door
	}
	pub fn get_south_door(&self) -> bool {
		self.has_south_door
	}
	pub fn get_adj(&self) -> String {
		self.adj.to_string()
	}
	pub fn get_description(&self) -> String {
		self.description.to_string()
	}
	pub fn get_item(&self) -> i32 {
		if self.room_has_cream {
			1
		}
		else if self.room_has_coffee {
			2
		}
		else if self.room_has_sugar {
			3
		}
		else {
			0
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
	assert!(r.get_north_door())
}

#[test]
fn room_1_north_door() {
	/*Allocate a new room*/
	let r = Room::new(1);
	assert!(r.has_north_door);
	assert!(r.get_north_door())
}

#[test]
fn room_2_north_door() {
	/*Allocate a new room*/
	let r = Room::new(2);
	assert!(r.has_north_door);
	assert!(r.get_north_door())
}
#[test]
fn room_3_north_door() {
	/*Allocate a new room*/
	let r = Room::new(3);
	assert!(r.has_north_door);
	assert!(r.get_north_door())
}
#[test]
fn room_4_north_door() {
	/*Allocate a new room*/
	let r = Room::new(4);
	assert!(r.has_north_door);
	assert!(r.get_north_door())
}
#[test]
fn room_5_north_door() {
	/*Allocate a new room*/
	let r = Room::new(5);
	assert_eq!(r.has_north_door,false);
	assert_eq!(r.get_north_door(),false)
}
/*Test whether each room has the correct status for the south door*/
#[test]
fn room_0_south_door() {
	/*Allocate a new room*/
	let r = Room::new(0);
	assert_eq!(r.has_south_door,false);
	assert_eq!(r.get_south_door(),false)
}
#[test]
fn room_1_south_door() {
	/*Allocate a new room*/
	let r = Room::new(1);
	assert!(r.has_south_door);
	assert!(r.get_south_door())
}
#[test]
fn room_2_south_door() {
	/*Allocate a new room*/
	let r = Room::new(2);
	assert!(r.has_south_door);
	assert!(r.get_south_door())
}
#[test]
fn room_3_south_door() {
	/*Allocate a new room*/
	let r = Room::new(3);
	assert!(r.has_south_door);
	assert!(r.get_south_door())
}
#[test]
fn room_4_south_door() {
	/*Allocate a new room*/
	let r = Room::new(4);
	assert!(r.has_south_door);
	assert!(r.get_south_door())
}
#[test]
fn room_5_south_door() {
	/*Allocate a new room*/
	let r = Room::new(5);
	assert!(r.has_south_door);
	assert!(r.get_south_door())
}
/*Test whether each room has the correct descriptionr*/
#[test]
fn room_0_description() {
	/*Allocate a new room*/
	let r = Room::new(0);
	assert_eq!(r.description,"a statue of Bill Laboon");
	assert_eq!(r.get_description(),"a statue of Bill Laboon");
}
#[test]
fn room_1_description() {
	/*Allocate a new room*/
	let r = Room::new(1);
	assert_eq!(r.description,"Amazon's best-seller, \"A Friendly Introduction to Software Testing\" by THE Bill Laboon");
	assert_eq!(r.get_description(),"Amazon's best-seller, \"A Friendly Introduction to Software Testing\" by THE Bill Laboon");

}
#[test]
fn room_2_description() {
	/*Allocate a new room*/
	let r = Room::new(2);
	assert_eq!(r.description,"an autographed photo of Bill Laboon");
	assert_eq!(r.get_description(),"an autographed photo of Bill Laboon");

}
#[test]
fn room_3_description() {
	/*Allocate a new room*/
	let r = Room::new(3);
	assert_eq!(r.description,"\"Hackin' Fellow\" on repeat 'cause it's such an amazing song");
	assert_eq!(r.get_description(),"\"Hackin' Fellow\" on repeat 'cause it's such an amazing song");

}
#[test]
fn room_4_description() {
	/*Allocate a new room*/
	let r = Room::new(4);
	assert_eq!(r.description,"a broken metronome that is constantly on high");
	assert_eq!(r.get_description(),"a broken metronome that is constantly on high");
}
#[test]
fn room_5_description() {
	/*Allocate a new room*/
	let r = Room::new(5);
	assert_eq!(r.description,"A number of Cats from the billion dollar enterprise: Rent A Cat");

}
/*Test whether each room has the correct adjective*/
#[test]
fn room_0_adj() {
	/*Allocate a new room*/
	let r = Room::new(0);
	assert_eq!(r.adj,"Inspirational");
	assert_eq!(r.get_adj(),"Inspirational");
}
#[test]
fn room_1_adj() {
	/*Allocate a new room*/
	let r = Room::new(1);
	assert_eq!(r.adj,"Cool-Dude");
	assert_eq!(r.get_adj(),"Cool-Dude");
}
#[test]
fn room_2_adj() {
	/*Allocate a new room*/
	let r = Room::new(2);
	assert_eq!(r.adj,"Chili-Pepper");
	assert_eq!(r.get_adj(),"Chili-Pepper");
}
#[test]
fn room_3_adj() {
	/*Allocate a new room*/
	let r = Room::new(3);
	assert_eq!(r.adj,"Smart");
	assert_eq!(r.get_adj(),"Smart");
}
#[test]
fn room_4_adj() {
	/*Allocate a new room*/
	let r = Room::new(4);
	assert_eq!(r.adj,"Fun");
	assert_eq!(r.get_adj(),"Fun");
}
#[test]
fn room_5_adj() {
	/*Allocate a new room*/
	let r = Room::new(5);
	assert_eq!(r.adj, "Hilarious");
	assert_eq!(r.get_adj(),"Hilarious");
}
/*verify the contructor and the getter for items return the correct object*/
#[test]
fn room_0_item() {
	/*Allocate a new room*/
	let r = Room::new(0);
	assert_eq!(r.room_has_cream,true);
	assert_eq!(r.room_has_coffee,false);
	assert_eq!(r.room_has_sugar,false);
	assert_eq!(r.get_item(),1);
}
#[test]
fn room_1_item() {
	/*Allocate a new room*/
	let r = Room::new(1);
	assert_eq!(r.room_has_cream,false);
	assert_eq!(r.room_has_coffee,true);
	assert_eq!(r.room_has_sugar,false);
	assert_eq!(r.get_item(),2);
}
#[test]
fn room_2_item() {
	/*Allocate a new room*/
	let r = Room::new(2);
	assert_eq!(r.room_has_cream,false);
	assert_eq!(r.room_has_coffee,false);
	assert_eq!(r.room_has_sugar,false);
	assert_eq!(r.get_item(),0);
}
#[test]
fn room_3_item() {
	/*Allocate a new room*/
	let r = Room::new(3);
	assert_eq!(r.room_has_cream,false);
	assert_eq!(r.room_has_coffee,false);
	assert_eq!(r.room_has_sugar,false);
	assert_eq!(r.get_item(),0);
}
#[test]
fn room_4_item() {
	/*Allocate a new room*/
	let r = Room::new(4);
	assert_eq!(r.room_has_cream,false);
	assert_eq!(r.room_has_coffee,false);
	assert_eq!(r.room_has_sugar,false);
	assert_eq!(r.get_item(),0);
}
#[test]
fn room_5_item() {
	/*Allocate a new room*/
	let r = Room::new(5);
	assert_eq!(r.room_has_cream,false);
	assert_eq!(r.room_has_coffee,false);
	assert_eq!(r.room_has_sugar,true);
	assert_eq!(r.get_item(),3);
}
