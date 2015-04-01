//! This crate contains one and only one thing: CHIP-8 emulator.
//!
//! The detailed specification for CHIP-8 can be found on [Cowgod's Chip-8 Technical Reference][Cowgod], compiled by Thomas P. Greene, or [CHIP-8 Wikipedia][wiki].
//!
//! [Cowgod]: http://devernay.free.fr/hacks/chip8/C8TECH10.HTM "Cowgod's Chip-8 Technical Reference"
//! [wiki]: http://en.wikipedia.org/wiki/CHIP-8

#![feature(core)]

extern crate rand;

use rand::random;
use std::num::wrapping::WrappingOps;

/// A bare CHIP-8 emulator.
pub struct Chip8 {
  /// index register
  i: usize,
  /// program counter
  pc: usize,
  /// current opcode
  opcode: u16,
  /// registers from v0 to v16
  v: [u8; 16],
  /// memory
  memory: [u8; 4096],
  /// display
  display: [u8; 2048],
  /// draw flag
  pub draw_flag: bool,
  /// delay timer
  delay_timer: u8,
  /// sound timer
  sound_timer: u8,
  /// call stack
  stack: [usize; 16],
  /// stack pointer
  sp: usize,
  /// key pressed -> true
  pub key: [bool; 16],
}

impl Chip8 {
  /// Creates a new Chip8.
  pub fn new() -> Chip8 {
    let mut chip8 = Chip8 {
      // program starts at 0x200.
      i: 0x200,
      pc: 0x200,
      opcode: 0,
      v: [0; 16],
      memory: [0; 4096],
      display: [0; 2048],
      draw_flag: false,
      delay_timer: 0,
      sound_timer: 0,
      stack: [0; 16],
      sp: 0,
      key: [false; 16],
    };

    for i in 0..80 {
      chip8.memory[i] = FONTSET[i];
    }

    chip8
  }

  /// Loads an array of bytes (program) into the memory.
  ///
  /// It does not reset any internal states.
  ///
  /// # Panics
  ///
  /// Panics if the program is longer than intended (4096 - 0x200 = 3584).
  pub fn load(&mut self, program: &[u8]) {
    let len_prog = program.len();
    for i in 0..len_prog {
      self.memory[i + 0x200] = program[i];
    }
  }
  /// Returns read-only graphics
  pub fn get_display(&self) -> &[u8; 2048] {
    &self.display
  }

  /// Returns whether the emulator is making sound or not
  pub fn get_sound(&self) -> bool {
    self.sound_timer > 0
  }

  /// Emulate one opcode.
  pub fn update(&mut self) {
    self.opcode = (self.memory[self.pc] as u16) << 8 | (self.memory[self.pc + 1] as u16);

    match self.opcode & 0xF000 {
      0x0000 => self.op_0xxx(),
      0x1000 => self.op_1xxx(),
      0x2000 => self.op_2xxx(),
      0x3000 => self.op_3xxx(),
      0x4000 => self.op_4xxx(),
      0x5000 => self.op_5xxx(),
      0x6000 => self.op_6xxx(),
      0x7000 => self.op_7xxx(),
      0x8000 => self.op_8xxx(),
      0x9000 => self.op_9xxx(),
      0xA000 => self.op_Axxx(),
      0xB000 => self.op_Bxxx(),
      0xC000 => self.op_Cxxx(),
      0xD000 => self.op_Dxxx(),
      0xE000 => self.op_Exxx(),
      0xF000 => self.op_Fxxx(),
      _ => wrong_opcode(self.opcode, self.pc),
    }

    if self.sound_timer > 0 {
      self.sound_timer -= 1;
    }
    if self.delay_timer > 0 {
      self.delay_timer -= 1;
    }
  }

  fn op_0xxx(&mut self) {
    match self.opcode & 0x0FFF {
      0x00E0 => {
        for i in &mut self.display[..] {
          *i = 0;
        }
        self.draw_flag = true;
      },
      0x00EE => {
        self.sp -= 1;
        self.pc = self.stack[self.sp];
      },
      _ => { /* nop, ignored */ },
    }
    self.pc += 2;
  }

  fn op_1xxx(&mut self) {
    self.pc = self.op_nnn() as usize;
  }

  fn op_2xxx(&mut self) {
    self.stack[self.sp] = self.pc;
    self.sp += 1;
    self.pc = self.op_nnn() as usize;
  }

  fn op_3xxx(&mut self) {
    if self.v[self.op_x()] == self.op_kk() {
      self.pc += 2;
    }
    self.pc += 2;
  }

  fn op_4xxx(&mut self) {
    if self.v[self.op_x()] != self.op_kk() {
      self.pc += 2;
    }
    self.pc += 2;
  }

  fn op_5xxx(&mut self) {
    match self.opcode & 0x000F {
      0x0000 => {
        if self.v[self.op_x()] == self.v[self.op_y()] {
          self.pc += 2;
        }
      },
      _ => wrong_opcode(self.opcode, self.pc),
    }
    self.pc += 2;
  }

  fn op_6xxx(&mut self) {
    self.v[self.op_x()] = self.op_kk();
    self.pc += 2;
  }

  fn op_7xxx(&mut self) {
    self.v[self.op_x()] += self.op_kk();
    self.pc += 2;
  }

  fn op_8xxx(&mut self) {
    match self.op_n() {
      0x0 => {
        self.v[self.op_x()] = self.v[self.op_y()];
      },
      0x1 => {
        self.v[self.op_x()] |= self.v[self.op_y()];
      },
      0x2 => {
        self.v[self.op_x()] &= self.v[self.op_y()];
      },
      0x3 => {
        self.v[self.op_x()] ^= self.v[self.op_y()];
      },
      0x4 => {
        match self.v[self.op_x()].checked_add(self.v[self.op_y()]) {
          Some(n) => {
            self.v[self.op_x()] = n;
            self.v[0xF] = 0;
          },
          None => {
            self.v[self.op_x()] = self.v[self.op_x()].wrapping_add(self.v[self.op_y()]);
            self.v[0xF] = 1;
          },
        }
      },
      0x5 => {
        if self.v[self.op_x()] > self.v[self.op_y()] {
          self.v[self.op_x()] -= self.v[self.op_y()];
          self.v[0xF] = 1;
        }
        else {
          self.v[self.op_x()] = self.v[self.op_x()].wrapping_sub(self.v[self.op_y()]);
          self.v[0xF] = 0;
        }
      },
      0x6 => {
        self.v[0xF] = self.v[self.op_x()] % 2;
        self.v[self.op_x()] /= 2;
      },
      0x7 => {
        if self.v[self.op_y()] > self.v[self.op_x()] {
          self.v[self.op_x()] = self.v[self.op_y()] - self.v[self.op_x()];
          self.v[0xF] = 1;
        }
        else {
          self.v[self.op_x()] = self.v[self.op_y()].wrapping_sub(self.v[self.op_x()]);
          self.v[0xF] = 0;
        }
      },
      0xE => {
        self.v[0xF] = self.v[self.op_x()] >> 7;
        self.v[self.op_x()] = self.v[self.op_x()].wrapping_mul(2);
      },
      _ => wrong_opcode(self.opcode, self.pc),
    }
    self.pc += 2;
  }

  fn op_9xxx(&mut self) {
    if self.v[self.op_x()] != self.v[self.op_y()] {
      self.pc += 2;
    }
    self.pc += 2;
  }

  #[allow(non_snake_case)]
  fn op_Axxx(&mut self) {
    self.i = self.op_nnn() as usize;
    self.pc += 2;
  }

  #[allow(non_snake_case)]
  fn op_Bxxx(&mut self) {
    self.pc = (self.op_nnn() + self.v[0x0] as u16) as usize;
  }

  #[allow(non_snake_case)]
  fn op_Cxxx(&mut self) {
    self.v[self.op_x()] = random::<u8>() & self.op_kk();
    self.pc += 2;
  }

  #[allow(non_snake_case)]
  fn op_Dxxx(&mut self) {
    self.v[0xF] = 0;

    for y in 0..self.op_n() {
      for x in 0..8 {
        if (self.memory[self.i + y as usize] & 0x80 >> x) != 0 {
          let pos = (self.v[self.op_y()] as usize + y as usize) * 64 + self.v[self.op_x()] as usize + x;
          if self.display[pos] == 1 {
            self.v[0xF] = 1;
          }
          self.display[pos] ^= 1;
        }
      }
    }
    self.draw_flag = true;

    self.pc += 2;
  }

  #[allow(non_snake_case)]
  fn op_Exxx(&mut self) {
    match self.op_kk() {
      0x9E => {
        if self.key[self.v[self.op_x()] as usize] {
          self.pc += 2;
        }
      },
      0xA1 => {
        if !self.key[self.v[self.op_x()] as usize] {
          self.pc += 2;
        }
      },
      _ => wrong_opcode(self.opcode, self.pc),
    }
    self.pc += 2;
  }

  #[allow(non_snake_case)]
  fn op_Fxxx(&mut self) {
    match self.op_kk() {
      0x07 => {
        self.v[self.op_x()] = self.delay_timer;
        self.pc += 2;
      },
      0x0A => {
        for i in 0..16 {
          if self.key[i] {
            self.v[self.op_x()] = i as u8;
            self.pc += 2;
            break;
          }
        }
      },
      0x15 => {
        self.delay_timer = self.v[self.op_x()];
        self.pc += 2;
      },
      0x18 => {
        self.sound_timer = self.v[self.op_x()];
        self.pc += 2;
      },
      0x1E => {
        self.i += self.v[self.op_x()] as usize;
        self.pc += 2;
      },
      0x29 => {
        self.i = self.v[self.op_x()] as usize * 5;
        self.pc += 2;
      },
      0x33 => {
        let vx = self.v[self.op_x()];
        self.memory[self.i] = vx / 100;
        self.memory[self.i + 1] = (vx % 100) / 10;
        self.memory[self.i + 2] = vx % 10;
        self.pc += 2;
      },
      0x55 => {
        for i in 0..(self.op_x() + 1) {
          self.memory[self.i + i] = self.v[i];
        }
        self.pc += 2;
      },
      0x65 => {
        for i in 0..(self.op_x() + 1) {
          self.v[i] = self.memory[self.i + i];
        }
        self.pc += 2;
      },
      _ => wrong_opcode(self.opcode, self.pc),
    }
  }

  fn op_nnn(&self) -> u16 {
    self.opcode & 0x0FFF
  }

  fn op_kk(&self) -> u8 {
    (self.opcode & 0x00FF) as u8
  }

  fn op_x(&self) -> usize {
    ((self.opcode & 0x0F00) >> 8) as usize
  }

  fn op_y(&self) -> usize {
    ((self.opcode & 0x00F0) >> 4) as usize
  }

  fn op_n(&self) -> u8 {
    (self.opcode & 0x000F) as u8
  }
}

fn wrong_opcode(opcode: u16, pc: usize) {
  println!("Wrong opcode: {:x} at pc: {:x}", opcode, pc);
}

static FONTSET: [u8; 80] = [
  0xF0, 0x90, 0x90, 0x90, 0xF0, // 0
  0x20, 0x60, 0x20, 0x20, 0x70, // 1
  0xF0, 0x10, 0xF0, 0x80, 0xF0, // 2
  0xF0, 0x10, 0xF0, 0x10, 0xF0, // 3
  0x90, 0x90, 0xF0, 0x10, 0x10, // 4
  0xF0, 0x80, 0xF0, 0x10, 0xF0, // 5
  0xF0, 0x80, 0xF0, 0x90, 0xF0, // 6
  0xF0, 0x10, 0x20, 0x40, 0x40, // 7
  0xF0, 0x90, 0xF0, 0x90, 0xF0, // 8
  0xF0, 0x90, 0xF0, 0x10, 0xF0, // 9
  0xF0, 0x90, 0xF0, 0x90, 0x90, // A
  0xE0, 0x90, 0xE0, 0x90, 0xE0, // B
  0xF0, 0x80, 0x80, 0x80, 0xF0, // C
  0xE0, 0x90, 0x90, 0x90, 0xE0, // D
  0xF0, 0x80, 0xF0, 0x80, 0xF0, // E
  0xF0, 0x80, 0xF0, 0x80, 0x80, // F
];

#[cfg(test)]
mod tests;
