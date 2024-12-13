#[derive(PartialEq, Eq, Debug, Clone, Copy)]
pub struct Location {
    // location along width of board
    pub x: usize,
    // location along height of board
    pub y: usize,
}

impl Location {
    pub fn new(x: usize, y: usize) -> Self {
        Self { x, y }
    }

    pub fn encode(&self) -> String {
        let base_int = 'a' as usize;
        let char_int = base_int + self.x;
        let char_value = char::from_u32(char_int as u32)
            .expect("Encoding location failed because x position could not be parsed to a char");

        format!("{}{}", char_value, self.y + 1)
    }

    // given the encoding of a position, e.g. e4, compute a location
    pub fn decode(encoded: &str) -> Result<Self, String> {
        let encoded_lower = encoded.to_lowercase();
        let base_int = 'a' as usize;
        let char_int = encoded_lower.chars().next().unwrap() as usize;
        let x = char_int - base_int;
        let y = encoded[1..]
            .parse::<usize>()
            .map_err(|_| "failed parsing second part of location encoding as an integer")?
            - 1;

        Ok(Location { x, y })
    }
}
