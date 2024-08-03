#[derive(Clone, Copy)]
pub enum Options {
  I,
  II,
  III,
  IV,
  V,
  VI,
  VII,
  VIII,
}

#[derive(Clone)]
pub struct Rotors {
  outputs: Vec<[usize; 26]>,
  notches: Vec<Vec<usize>>,
  offsets: Vec<usize>,
  count: usize,
}

impl Rotors {
  pub fn new() -> Self {
    Rotors {
      count: 0,
      outputs: vec![],
      notches: vec![],
      offsets: vec![],
    }
  }

  pub fn signal_right_to_left(&mut self, value: usize) -> usize {
    self.rotate();

    println!("{:?}", self.offsets);

    let mut current = value;

    for (rotor, &offset) in self.outputs.iter().zip(self.offsets.iter()).rev() {
        current = rotor[(current + offset) % 26];
    }

    current
  }

  pub fn signal_left_to_right(&self, value: usize) -> usize {
    let mut current = value;

    for (i, &x) in self.outputs.iter().zip(self.offsets.iter()) {
      // Find the position directly
      let temp = i.iter()
        .position(|&v| v == current)
        .unwrap_or(0);

      // Calculate new value and ensure it's within the range [0, 25]
      current = (temp + 26 - x) % 26;
    }

    current
  }

  pub fn set_rotors(&mut self, selection: &[Options]) {
    self.count = selection.len();

    for &rotor in selection {
      self.outputs.push(self.get_rotor(rotor));
      self.notches.push(self.get_notches(rotor));
    }
  }

  pub fn set_offsets(&mut self, offsets: &[usize]) {
    for &offset in offsets {
      self.offsets.push(offset % 26);
    }
  }

  fn rotate(&mut self) {
    for i in (0..self.offsets.len()).rev() {

      self.offsets[i] = (self.offsets[i] + 1) % 26;

      let  rotate_next = self.notches[i].contains(&self.offsets[i]);

      if !rotate_next {
        return;
      }
    }
  }

  fn get_rotor(&self, option: Options) -> [usize; 26] {
    match option {
      // E K M F L G D Q V Z N T O W Y H X U S P A I B R C J
      Options::I => [4, 10, 12, 5, 11, 6, 3, 16, 21, 25, 13, 19, 14, 22, 24, 7, 23, 20, 18, 15, 0, 8, 1, 17, 2, 9],
      // A J D K S I R U X B L H W T M C Q G Z N P Y F V O E
      Options::II => [0, 9, 3, 10, 18, 8, 17, 20, 23, 1, 11, 7, 22, 19, 12, 2, 16, 6, 25, 13, 15, 24, 5, 21, 14, 4],
      // B D F H J L C P R T X V Z N Y E I W G A K M U S Q O
      Options::III => [1, 3, 5, 7, 9, 11, 2, 15, 17, 19, 23, 21, 25, 13, 24, 4, 8, 22, 6, 0, 10, 12, 20, 18, 16, 14],
      // E S O V P Z J A Y Q U I R H X L N F T G K D C M W B
      Options::IV => [4, 18, 14, 21, 15, 25, 9, 0, 24, 16, 20, 8, 17, 7, 23, 11, 13, 5, 19, 6, 10, 3, 2, 12, 22, 1],
      // V Z B R G I T Y U P S D N H L X A W M J Q O F E C K
      Options::V => [21, 25, 1, 17, 6, 8, 19, 24, 20, 15, 18, 3, 13, 7, 11, 23, 0, 22, 12, 9, 16, 14, 5, 4, 2, 10],
      // J P G V O U M F Y Q B E N H Z R D K A S X L I C T W
      Options::VI => [9, 15, 6, 21, 14, 20, 12, 5, 24, 16, 1, 4, 13, 7, 25, 17, 3, 10, 0, 18, 23, 11, 8, 2, 19, 22],
      // N Z J H G R C X M Y S W B O U F A I V L P E K Q D T
      Options::VII => [3, 25, 9, 7, 6, 17, 2, 23, 12, 24, 18, 22, 1, 14, 20, 5, 0, 8, 21, 11, 15, 4, 10, 16, 3, 19],
      // F K Q H T L X O C B J S P D Z R A M E W N I U Y G V
      Options::VIII => [5, 10, 16, 7, 19, 11, 23, 14, 2, 1, 9, 18, 15, 3, 25, 17, 0, 12, 4, 22, 13, 8, 20, 24, 6, 21],
    }
  }

  fn get_notches(&self, option: Options) -> Vec<usize> {
    match option {
      // R
      Options::I => vec![17],
      // F
      Options::II => vec![5],
      // W
      Options::III => vec![22],
      // K
      Options::IV => vec![10],
      // A
      Options::V => vec![0],
      // A, N
      Options::VI | Options::VII | Options::VIII => vec![0, 13],
    }
  }
}
