#[derive(Clone, Copy)]
pub enum Rotors {
  I,
  II,
  III,
  IV,
  V,
  VI,
  VII,
  VIII,
}

pub enum Reflector {
  B,
  C,
}

#[derive(Clone)]
pub struct Enigma {
  plugboard: [usize; 26],
  rotors: [[usize; 26]; 3],
  reflector: [usize; 26],
  rotor_notches: [Vec<usize>; 3],
  rotor_offsets: [usize; 3],
}

impl Enigma {

  pub fn new(plugboard: [usize; 26], rotors: [Rotors; 3], reflector: Reflector) -> Self {
    Enigma {
      plugboard,
      rotors: get_rotors(rotors),
      reflector: get_reflector(reflector),
      rotor_notches: get_notches(rotors),
      rotor_offsets: [0; 3]
    }
  }

  pub fn set_offsets(&mut self, offsets: &[usize]) {
    for (i, &offset) in offsets.iter().enumerate() {
      self.rotor_offsets[i] = offset % 26;
    }
  }

  pub fn rotor_right_to_left(&mut self, value: usize) -> usize {
    self.rotate();

    println!("{:?}", self.rotor_offsets);

    let mut current = value;

    for (rotor, &offset) in self.rotors.iter().zip(self.rotor_offsets.iter()).rev() {
      current = rotor[(current + offset) % 26];
    }

    current
  }

  pub fn rotor_left_to_right(&self, value: usize) -> usize {
    let mut current = value;

    for (i, &x) in self.rotors.iter().zip(self.rotor_offsets.iter()) {
      // Find the position directly
      let temp = i.iter()
        .position(|&v| v == current)
        .unwrap_or(0);

      // Calculate new value and ensure it's within the range [0, 25]
      current = (temp + 26 - x) % 26;
    }

    current
  }


  fn rotate(&mut self) {
    for i in (0..3).rev() {

      self.rotor_offsets[i] = (self.rotor_offsets[i] + 1) % 26;

      let  rotate_next = self.rotor_notches[i].contains(&self.rotor_offsets[i]);

      if !rotate_next {
        return;
      }
    }
  }
}

fn get_rotors(rotor_options: [Rotors; 3]) -> [[usize; 26]; 3] {
  let mut rotors: [[usize; 26]; 3] = [[0; 26]; 3];

  for (i, &rotor) in rotor_options.iter().take(3).enumerate() {
    rotors[i] = get_rotor(rotor);
  }

  rotors
}

fn get_rotor(rotor: Rotors) -> [usize; 26] {
  match rotor {
    // E K M F L G D Q V Z N T O W Y H X U S P A I B R C J
    Rotors::I => [4, 10, 12, 5, 11, 6, 3, 16, 21, 25, 13, 19, 14, 22, 24, 7, 23, 20, 18, 15, 0, 8, 1, 17, 2, 9],
    // A J D K S I R U X B L H W T M C Q G Z N P Y F V O E
    Rotors::II => [0, 9, 3, 10, 18, 8, 17, 20, 23, 1, 11, 7, 22, 19, 12, 2, 16, 6, 25, 13, 15, 24, 5, 21, 14, 4],
    // B D F H J L C P R T X V Z N Y E I W G A K M U S Q O
    Rotors::III => [1, 3, 5, 7, 9, 11, 2, 15, 17, 19, 23, 21, 25, 13, 24, 4, 8, 22, 6, 0, 10, 12, 20, 18, 16, 14],
    // E S O V P Z J A Y Q U I R H X L N F T G K D C M W B
    Rotors::IV => [4, 18, 14, 21, 15, 25, 9, 0, 24, 16, 20, 8, 17, 7, 23, 11, 13, 5, 19, 6, 10, 3, 2, 12, 22, 1],
    // V Z B R G I T Y U P S D N H L X A W M J Q O F E C K
    Rotors::V => [21, 25, 1, 17, 6, 8, 19, 24, 20, 15, 18, 3, 13, 7, 11, 23, 0, 22, 12, 9, 16, 14, 5, 4, 2, 10],
    // J P G V O U M F Y Q B E N H Z R D K A S X L I C T W
    Rotors::VI => [9, 15, 6, 21, 14, 20, 12, 5, 24, 16, 1, 4, 13, 7, 25, 17, 3, 10, 0, 18, 23, 11, 8, 2, 19, 22],
    // N Z J H G R C X M Y S W B O U F A I V L P E K Q D T
    Rotors::VII => [3, 25, 9, 7, 6, 17, 2, 23, 12, 24, 18, 22, 1, 14, 20, 5, 0, 8, 21, 11, 15, 4, 10, 16, 3, 19],
    // F K Q H T L X O C B J S P D Z R A M E W N I U Y G V
    Rotors::VIII => [5, 10, 16, 7, 19, 11, 23, 14, 2, 1, 9, 18, 15, 3, 25, 17, 0, 12, 4, 22, 13, 8, 20, 24, 6, 21],
  }
}

fn get_notches(rotors: [Rotors; 3]) -> [Vec<usize>; 3] {
  let mut notches = [vec![], vec![], vec![]];

  for (i, &rotor) in rotors.iter().take(3).enumerate() {
    notches[i] = get_notch(rotor);
  }

  notches
}

fn get_notch(rotor: Rotors) -> Vec<usize> {
  match rotor {
    // R
    Rotors::I => vec![17],
    // F
    Rotors::II => vec![5],
    // W
    Rotors::III => vec![22],
    // K
    Rotors::IV => vec![10],
    // A
    Rotors::V => vec![0],
    // A, N
    Rotors::VI | Rotors::VII | Rotors::VIII => vec![0, 13],
  }
}

fn get_reflector(reflector: Reflector) -> [usize; 26] {
    match reflector {
      // Y R U H Q S L D P X N G O K M I E B F Z C W V J A T
      Reflector::B => [24, 17, 20, 7, 16, 18, 11, 3, 15, 23, 13, 6, 14, 10, 12, 8, 4, 1, 5, 25, 2, 22, 21, 9, 0, 19],
      // F V P J I A O Y E D R Z X W G C T K U Q S B H R G L
      Reflector::C => [5, 21, 15, 9, 8, 0, 14, 24, 4, 3, 17, 25, 23, 22, 6, 2, 19, 10, 20, 16, 18, 1, 7, 17, 6, 11]
    }
}