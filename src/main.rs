use crate::rotors::{Options, Rotors};

mod rotors;
mod plugboard;
mod reflector;

const ALPHABET: [char; 26] = [
'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L', 'M',
'N', 'O', 'P', 'Q', 'R', 'S', 'T', 'U', 'V', 'W', 'X', 'Y', 'Z'
];

fn main() {
    let mut rotors: Rotors = Rotors::new();

    rotors.set_rotors(&[Options::III, Options::II, Options::VI]);
    rotors.set_offsets(&[0,0,0]);
}

fn convert_num_to_letter(num: usize) -> char{
    ALPHABET[num % 26]
}

fn convert_letter_to_num(letter: char) -> usize{
    ALPHABET.iter()
      .position(|&l| l == letter)
      .unwrap_or(0)
}