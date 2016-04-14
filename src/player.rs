#[test]
fn it_works() {
    assert_eq!(4, 4);
}

#[test]
#[should_panic(expected = "assertion failed")] /*Mark that this test case should fail*/
fn test_constructor() {
 let p = Player::new();
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
