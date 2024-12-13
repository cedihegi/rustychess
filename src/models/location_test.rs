use super::location::Location;

#[test]
fn encode_success_1() {
    // arrange
    let location = Location::new(6, 7);
    // action
    let encoded = location.encode();
    // assert
    assert_eq!(encoded, "g8")
}

#[test]
fn encode_success_2() {
    // arrange
    let location = Location::new(0, 0);
    // action
    let encoded = location.encode();
    // assert
    assert_eq!(encoded, "a1")
}

#[test]
fn decode_success() {
    // arrange
    let input = "b5".to_string();
    let expected_location = Location::new(1, 4);
    // action
    let decoded = Location::decode(&input);
    // assert
    assert_eq!(decoded, Ok(expected_location));
}
