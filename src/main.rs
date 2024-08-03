use crate::enigma::{Enigma, Reflector, Rotors};

mod enigma;

const ALPHABET: [char; 26] = [
'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L', 'M',
'N', 'O', 'P', 'Q', 'R', 'S', 'T', 'U', 'V', 'W', 'X', 'Y', 'Z'
];

fn main() {
    let mut machine: Enigma = Enigma::new(
        [0,1,2,3,4,5,6,7,8,9,10,11,12,13,14,15,16,17,18,19,20,21,22,23,24,25],
        [Rotors::III, Rotors::II, Rotors::I],
        Reflector::B);

    machine.set_offsets(&[0,0,0]);

    let mut letter = 'A';
    let mut value = convert_letter_to_num(letter);

    println!("{letter} : {value}");

    value = machine.rotor_right_to_left(value);
    letter = convert_num_to_letter(value);

    println!("{letter} : {value}");

    value = machine.rotor_left_to_right(value);
    letter = convert_num_to_letter(value);

    println!("{letter} : {value}");
}

fn convert_num_to_letter(num: usize) -> char{
    ALPHABET[num % 26]
}

fn convert_letter_to_num(letter: char) -> usize{
    ALPHABET.iter()
      .position(|&l| l == letter)
      .unwrap_or(0)
}