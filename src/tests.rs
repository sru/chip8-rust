// Since opcodes are uppercased.
#![allow(non_snake_case)]

use super::Chip8;
use rand::random;

#[test]
#[should_panic]
fn load_limit() {
  let mut chip8 = Chip8::new();
  chip8.load(&[0; 3585]);
}

#[test]
fn delay_timer() {
  let mut chip8 = Chip8::new();

  chip8.delay_timer = 2;

  chip8.update();
  assert_eq!(1, chip8.delay_timer);

  chip8.update();
  assert_eq!(0, chip8.delay_timer);

  chip8.update();
  assert_eq!(0, chip8.delay_timer);
}

#[test]
fn sound() {
  let mut chip8 = Chip8::new();

  chip8.sound_timer = 2;

  chip8.update();
  assert_eq!(1, chip8.sound_timer);
  assert_eq!(true, chip8.get_sound());

  chip8.update();
  assert_eq!(0, chip8.sound_timer);
  assert_eq!(false, chip8.get_sound());

  chip8.update();
  assert_eq!(0, chip8.sound_timer);
}

#[test]
fn op_0nnn() {
  // ignored
}

#[test]
fn op_00E0() {
  let mut chip8 = Chip8::new();

  let clean: [u8; 2048] = [0; 2048];

  for i in 0..2048 {
    chip8.display[i] = random() % 2;
  }

  chip8.load(&[0x00, 0xE0]);

  chip8.update();
  assert_eq!(&clean[..], &chip8.display[..]);
}

#[test]
fn op_00EE() {
  let mut chip8 = Chip8::new();

  chip8.sp = 1;
  chip8.stack[0] = 0x300;

  chip8.load(&[0x00, 0xEE]);

  chip8.update();
  assert_eq!(0, chip8.sp);
  assert_eq!(0x302, chip8.pc);
}

#[test]
fn op_1nnn() {
  let mut chip8 = Chip8::new();

  chip8.load(&[0x13, 0x00]);

  chip8.update();
  assert_eq!(0x300, chip8.pc);
}

#[test]
fn op_2nnn() {
  let mut chip8 = Chip8::new();

  chip8.load(&[0x23, 0x00]);

  chip8.update();
  assert_eq!(1, chip8.sp);
  assert_eq!(0x200, chip8.stack[0]);
}

#[test]
fn op_3xkk() {
  let mut chip8 = Chip8::new();

  chip8.v[0x5] = 0xAB;

  chip8.load(&[0x35, 0x01, 0x35, 0xAB]);

  chip8.update();
  assert_eq!(0x202, chip8.pc);

  chip8.update();
  assert_eq!(0x206, chip8.pc);
}

#[test]
fn op_4xkk() {
  let mut chip8 = Chip8::new();

  chip8.v[0x5] = 0xAB;

  chip8.load(&[0x45, 0xAB, 0x45, 0x01]);

  chip8.update();
  assert_eq!(0x202, chip8.pc);

  chip8.update();
  assert_eq!(0x206, chip8.pc);
}

#[test]
fn op_5xy0() {
  let mut chip8 = Chip8::new();

  chip8.v[0x5] = 0xAB;

  chip8.load(&[0x55, 0x60, 0x55, 0x60]);

  chip8.update();
  assert_eq!(0x202, chip8.pc);

  chip8.v[0x6] = 0xAB;
  chip8.update();
  assert_eq!(0x206, chip8.pc);
}

#[test]
fn op_6xkk() {
  let mut chip8 = Chip8::new();

  chip8.load(&[0x65, 0xAB]);

  chip8.update();
  assert_eq!(0xAB, chip8.v[0x5]);
}

#[test]
fn op_7xkk() {
  let mut chip8 = Chip8::new();

  chip8.v[0x5] = 0xAB;

  chip8.load(&[0x75, 0x02]);

  chip8.update();
  assert_eq!(0xAD, chip8.v[0x5]);
}

#[test]
fn op_8xy0() {
  let mut chip8 = Chip8::new();

  chip8.v[0x5] = 0xAB;

  chip8.load(&[0x86, 0x50]);

  chip8.update();
  assert_eq!(0xAB, chip8.v[0x6]);
}

#[test]
fn op_8xy1() {
  let mut chip8 = Chip8::new();

  chip8.v[0x5] = 0b10001000;
  chip8.v[0x6] = 0b01100110;

  chip8.load(&[0x85, 0x61]);

  chip8.update();
  assert_eq!(0b11101110, chip8.v[0x5]);
}

#[test]
fn op_8xy2() {
  let mut chip8 = Chip8::new();

  chip8.v[0x5] = 0b11100111;
  chip8.v[0x6] = 0b10111101;

  chip8.load(&[0x85, 0x62]);

  chip8.update();
  assert_eq!(0b10100101, chip8.v[0x5]);
}

#[test]
fn op_8xy3() {
  let mut chip8 = Chip8::new();

  chip8.v[0x5] = 0b11100111;
  chip8.v[0x6] = 0b10101101;

  chip8.load(&[0x85, 0x63]);

  chip8.update();
  assert_eq!(0b01001010, chip8.v[0x5]);
}

#[test]
fn op_8xy4() {
  let mut chip8 = Chip8::new();

  chip8.v[0x5] = 0xFB;
  chip8.v[0x6] = 0x06;

  chip8.load(&[0x85, 0x64, 0x85, 0x64]);

  chip8.update();
  assert_eq!(0x01, chip8.v[0x5]);
  assert_eq!(1, chip8.v[0xF]);

  chip8.update();
  assert_eq!(0x07, chip8.v[0x5]);
  assert_eq!(0, chip8.v[0xF]);
}

#[test]
fn op_8xy5() {
  let mut chip8 = Chip8::new();

  chip8.v[0x5] = 0x08;
  chip8.v[0x6] = 0x06;

  chip8.load(&[0x85, 0x65, 0x85, 0x65]);

  chip8.update();
  assert_eq!(0x02, chip8.v[0x5]);
  assert_eq!(1, chip8.v[0xF]);

  chip8.update();
  assert_eq!(0xFC, chip8.v[0x5]);
  assert_eq!(0, chip8.v[0xF]);
}

#[test]
fn op_8xy6() {
  let mut chip8 = Chip8::new();

  chip8.v[0x5] = 0b00101001;

  chip8.load(&[0x85, 0x66, 0x85, 0x76]);

  chip8.update();
  assert_eq!(0b00010100, chip8.v[0x5]);
  assert_eq!(1, chip8.v[0xF]);

  chip8.update();
  assert_eq!(0b00001010, chip8.v[0x5]);
  assert_eq!(0, chip8.v[0xF]);
}

#[test]
fn op_8xy7() {
  let mut chip8 = Chip8::new();

  chip8.v[0x5] = 0x06;
  chip8.v[0x6] = 0x08;
  chip8.v[0x8] = 0x01;

  chip8.load(&[0x85, 0x67, 0x85, 0x87]);

  chip8.update();
  assert_eq!(0x02, chip8.v[0x5]);
  assert_eq!(1, chip8.v[0xF]);

  chip8.update();
  assert_eq!(0xFF, chip8.v[0x5]);
  assert_eq!(0, chip8.v[0xF]);
}

#[test]
fn op_8xyE() {
  let mut chip8 = Chip8::new();

  chip8.v[0x5] = 0b10011011;

  chip8.load(&[0x85, 0x0E, 0x85, 0xAE]);

  chip8.update();
  assert_eq!(0b00110110, chip8.v[0x5]);
  assert_eq!(1, chip8.v[0xF]);

  chip8.update();
  assert_eq!(0b01101100, chip8.v[0x5]);
  assert_eq!(0, chip8.v[0xF]);
}

#[test]
fn op_9xy0() {
  let mut chip8 = Chip8::new();

  chip8.v[0x5] = 0xAB;
  chip8.v[0x6] = 0xAB;

  chip8.load(&[0x95, 0x60, 0x95, 0x60]);

  chip8.update();
  assert_eq!(0x202, chip8.pc);

  chip8.v[0x6] = 0x00;
  chip8.update();
  assert_eq!(0x206, chip8.pc);
}

#[test]
fn op_Annn() {
  let mut chip8 = Chip8::new();

  chip8.load(&[0xA3, 0x00]);

  chip8.update();
  assert_eq!(0x300, chip8.i);
}

#[test]
fn op_Bnnn() {
  let mut chip8 = Chip8::new();

  chip8.v[0x0] = 0x05;

  chip8.load(&[0xB3, 0x00]);

  chip8.update();
  assert_eq!(0x305, chip8.pc);
}

#[test]
fn op_Cxkk() {
  // TODO: How do I test randomness?
}

#[test]
fn op_Dxyn() {
  let mut chip8 = Chip8::new();

  chip8.display[3] = 1;
  chip8.v[0x2] = 5;
  // 8
  chip8.i = 40;

  chip8.load(&[0xD0, 0x15, 0xD2, 0x15]);

  chip8.update();
  assert_eq!(&[1, 1, 1, 0], &chip8.display[64 * 0..64 * 0 + 4]);
  assert_eq!(&[1, 0, 0, 1], &chip8.display[64 * 1..64 * 1 + 4]);
  assert_eq!(&[1, 1, 1, 1], &chip8.display[64 * 2..64 * 2 + 4]);
  assert_eq!(&[1, 0, 0, 1], &chip8.display[64 * 3..64 * 3 + 4]);
  assert_eq!(&[1, 1, 1, 1], &chip8.display[64 * 4..64 * 4 + 4]);
  assert_eq!(1, chip8.v[0xF]);

  chip8.update();
  assert_eq!(&[1, 1, 1, 1], &chip8.display[64 * 0 + 5..64 * 0 + 9]);
  assert_eq!(&[1, 0, 0, 1], &chip8.display[64 * 1 + 5..64 * 1 + 9]);
  assert_eq!(&[1, 1, 1, 1], &chip8.display[64 * 2 + 5..64 * 2 + 9]);
  assert_eq!(&[1, 0, 0, 1], &chip8.display[64 * 3 + 5..64 * 3 + 9]);
  assert_eq!(&[1, 1, 1, 1], &chip8.display[64 * 4 + 5..64 * 4 + 9]);
  assert_eq!(0, chip8.v[0xF]);
}

#[test]
fn op_Ex9E() {
  let mut chip8 = Chip8::new();

  chip8.v[0x5] = 0x06;

  chip8.load(&[0xE5, 0x9E, 0xE5, 0x9E]);

  chip8.update();
  assert_eq!(0x202, chip8.pc);

  chip8.key[0x06] = true;
  chip8.update();
  assert_eq!(0x206, chip8.pc);
}

#[test]
fn op_ExA1() {
  let mut chip8 = Chip8::new();

  chip8.v[0x5] = 0x06;

  chip8.load(&[0xE5, 0xA1, 0xE5, 0xA1]);

  chip8.key[0x06] = true;
  chip8.update();
  assert_eq!(0x202, chip8.pc);

  chip8.key[0x06] = false;
  chip8.update();
  assert_eq!(0x206, chip8.pc);
}

#[test]
fn op_Fx07() {
  let mut chip8 = Chip8::new();

  chip8.delay_timer = 16;

  chip8.load(&[0xF5, 0x07]);

  chip8.update();
  assert_eq!(16, chip8.v[0x5]);
}

#[test]
fn op_Fx0A() {
  let mut chip8 = Chip8::new();

  chip8.load(&[0xF5, 0x0A]);

  chip8.update();
  assert_eq!(0x200, chip8.pc);

  chip8.key[0xC] = true;
  chip8.update();
  assert_eq!(0x202, chip8.pc);
  assert_eq!(0xC, chip8.v[0x5]);
}

#[test]
fn op_Fx15() {
  let mut chip8 = Chip8::new();

  chip8.v[0x5] = 63;

  chip8.load(&[0xF5, 0x15]);

  chip8.update();
  // already decreased by one.
  assert_eq!(62, chip8.delay_timer);
}

#[test]
fn op_Fx18() {
  let mut chip8 = Chip8::new();

  chip8.v[0x5] = 63;

  chip8.load(&[0xF5, 0x18]);

  chip8.update();
  // already decreased by one.
  assert_eq!(62, chip8.sound_timer);
}

#[test]
fn op_Fx1E() {
  let mut chip8 = Chip8::new();

  chip8.v[0x5] = 0xAB;

  chip8.load(&[0xF5, 0x1E]);

  chip8.update();
  assert_eq!(0x2AB, chip8.i);
}

#[test]
fn op_Fx29() {
  let mut chip8 = Chip8::new();

  chip8.v[0x5] = 0xA;

  chip8.load(&[0xF4, 0x29, 0xF5, 0x29]);

  chip8.update();
  assert_eq!(0x0, chip8.i);

  chip8.update();
  assert_eq!(50, chip8.i);
}

#[test]
fn op_Fx33() {
  let mut chip8 = Chip8::new();

  chip8.v[0x5] = 233;

  chip8.load(&[0xF5, 0x33]);

  chip8.update();
  assert_eq!(&[2, 3, 3], &chip8.memory[0x200..0x203]);
}

#[test]
fn op_Fx55() {
  let mut chip8 = Chip8::new();

  for i in &mut chip8.v[..6] {
    *i = random();
  }


  chip8.load(&[0xF5, 0x55]);

  chip8.update();
  assert_eq!(&chip8.v[..6], &chip8.memory[0x200..0x206]);
}

#[test]
fn op_Fx65() {
  let mut chip8 = Chip8::new();

  chip8.load(&[0xF5, 0x65, 0x22, 0x04, 0x30, 0x00, 0x00, 0xEE]);

  chip8.update();
  assert_eq!(&[0xF5, 0x65, 0x22, 0x04, 0x30, 0x00], &chip8.v[..6]);
}
