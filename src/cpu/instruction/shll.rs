use crate::cpu::{Cpu, CCR};
use anyhow::Result;

impl Cpu {
    pub(in super::super) fn shll_b(&mut self, opcode: u16) -> Result<usize> {
        let register = Cpu::get_nibble_opcode(opcode, 4)?;
        let src = self.read_rn_b(register)?;
        if src & 0x40 == 0x40 {
            self.write_ccr(CCR::N, 1)
        } else {
            self.write_ccr(CCR::N, 0);
        }
        if (src << 1) == 0 {
            self.write_ccr(CCR::Z, 1)
        } else {
            self.write_ccr(CCR::Z, 0);
        }
        self.write_ccr(CCR::V, 0);
        self.write_ccr(CCR::C, src >> 7);
        self.write_rn_b(register, src << 1)?;

        Ok(2)
    }

    pub(in super::super) fn shll_w(&mut self, opcode: u16) -> Result<usize> {
        let register = Cpu::get_nibble_opcode(opcode, 4)?;
        let src = self.read_rn_w(register)?;
        if src & 0x4000 == 0x4000 {
            self.write_ccr(CCR::N, 1)
        } else {
            self.write_ccr(CCR::N, 0);
        }
        if (src << 1) == 0 {
            self.write_ccr(CCR::Z, 1)
        } else {
            self.write_ccr(CCR::Z, 0);
        }
        self.write_ccr(CCR::V, 0);
        self.write_ccr(CCR::C, (src >> 15) as u8);
        self.write_rn_w(register, src << 1)?;

        Ok(2)
    }

    pub(in super::super) fn shll_l(&mut self, opcode: u16) -> Result<usize> {
        let register = Cpu::get_nibble_opcode(opcode, 4)?;
        let src = self.read_rn_l(register)?;
        if src & 0x40000000 == 0x40000000 {
            self.write_ccr(CCR::N, 1)
        } else {
            self.write_ccr(CCR::N, 0);
        }
        if (src << 1) == 0 {
            self.write_ccr(CCR::Z, 1)
        } else {
            self.write_ccr(CCR::Z, 0);
        }
        self.write_ccr(CCR::V, 0);
        self.write_ccr(CCR::C, (src >> 31) as u8);
        self.write_rn_l(register, src << 1)?;

        Ok(2)
    }
}

#[cfg(test)]
mod tests {
    use crate::cpu::Cpu;

    #[test]
    fn test_shll_b() {
        let mut cpu = Cpu::new();
        cpu.ccr = 0b00000111;

        cpu.bus.memory[0..2].copy_from_slice(&[0x10, 0x00]);
        cpu.write_rn_b(0, 0b0101_0101).unwrap();
        let opcode = cpu.fetch();
        let state = cpu.exec(opcode).unwrap();
        assert_eq!(state, 2);
        assert_eq!(cpu.ccr, 0b00001000);
        assert_eq!(cpu.read_rn_b(0).unwrap(), 0b1010_1010);

        let mut cpu = Cpu::new();
        cpu.ccr = 0b00000111;

        cpu.bus.memory[0..2].copy_from_slice(&[0x10, 0x0f]);
        cpu.write_rn_b(0xf, 0b0101_0101).unwrap();
        let opcode = cpu.fetch();
        let state = cpu.exec(opcode).unwrap();
        assert_eq!(state, 2);
        assert_eq!(cpu.ccr, 0b00001000);
        assert_eq!(cpu.read_rn_b(0xf).unwrap(), 0b1010_1010);

        // check CCR V
        let mut cpu = Cpu::new();
        cpu.ccr = 0b00001110;

        cpu.bus.memory[0..2].copy_from_slice(&[0x10, 0x00]);
        cpu.write_rn_b(0, 0b1010_1010).unwrap();
        let opcode = cpu.fetch();
        let state = cpu.exec(opcode).unwrap();
        assert_eq!(state, 2);
        assert_eq!(cpu.ccr, 0b00000001);
        assert_eq!(cpu.read_rn_b(0).unwrap(), 0b0101_0100);

        // check CCR C, Z
        let mut cpu = Cpu::new();
        cpu.ccr = 0b00001010;

        cpu.bus.memory[0..2].copy_from_slice(&[0x10, 0x00]);
        cpu.write_rn_b(0, 0b1000_0000).unwrap();
        let opcode = cpu.fetch();
        let state = cpu.exec(opcode).unwrap();
        assert_eq!(state, 2);
        assert_eq!(cpu.ccr, 0b00000101);
        assert_eq!(cpu.read_rn_b(0).unwrap(), 0);
    }

    #[test]
    fn test_shll_w() {
        let mut cpu = Cpu::new();
        cpu.ccr = 0b00000111;

        cpu.bus.memory[0..2].copy_from_slice(&[0x10, 0x10]);
        cpu.write_rn_w(0, 0b0101_0101_0101_0101).unwrap();
        let opcode = cpu.fetch();
        let state = cpu.exec(opcode).unwrap();
        assert_eq!(state, 2);
        assert_eq!(cpu.ccr, 0b00001000);
        assert_eq!(cpu.read_rn_w(0).unwrap(), 0b1010_1010_1010_1010);

        let mut cpu = Cpu::new();
        cpu.ccr = 0b00000111;

        cpu.bus.memory[0..2].copy_from_slice(&[0x10, 0x1f]);
        cpu.write_rn_w(0xf, 0b0101_0101_0101_0101).unwrap();
        let opcode = cpu.fetch();
        let state = cpu.exec(opcode).unwrap();
        assert_eq!(state, 2);
        assert_eq!(cpu.ccr, 0b00001000);
        assert_eq!(cpu.read_rn_w(0xf).unwrap(), 0b1010_1010_1010_1010);

        // check CCR V
        let mut cpu = Cpu::new();
        cpu.ccr = 0b00001110;

        cpu.bus.memory[0..2].copy_from_slice(&[0x10, 0x10]);
        cpu.write_rn_w(0, 0b1010_1010_1010_1010).unwrap();
        let opcode = cpu.fetch();
        let state = cpu.exec(opcode).unwrap();
        assert_eq!(state, 2);
        assert_eq!(cpu.ccr, 0b00000001);
        assert_eq!(cpu.read_rn_w(0).unwrap(), 0b0101_0101_0101_0100);

        // check CCR C, Z
        let mut cpu = Cpu::new();
        cpu.ccr = 0b00001010;

        cpu.bus.memory[0..2].copy_from_slice(&[0x10, 0x10]);
        cpu.write_rn_w(0, 0b1000_0000_0000_0000).unwrap();
        let opcode = cpu.fetch();
        let state = cpu.exec(opcode).unwrap();
        assert_eq!(state, 2);
        assert_eq!(cpu.ccr, 0b00000101);
        assert_eq!(cpu.read_rn_w(0).unwrap(), 0);
    }

    #[test]
    fn test_shll_l() {
        let mut cpu = Cpu::new();
        cpu.ccr = 0b00000111;

        cpu.bus.memory[0..2].copy_from_slice(&[0x10, 0x30]);
        cpu.write_rn_l(0, 0b0101_0101_0101_0101_0101_0101_0101_0101)
            .unwrap();
        let opcode = cpu.fetch();
        let state = cpu.exec(opcode).unwrap();
        assert_eq!(state, 2);
        assert_eq!(cpu.ccr, 0b00001000);
        assert_eq!(
            cpu.read_rn_l(0).unwrap(),
            0b1010_1010_1010_1010_1010_1010_1010_1010
        );

        let mut cpu = Cpu::new();
        cpu.ccr = 0b00000111;

        cpu.bus.memory[0..2].copy_from_slice(&[0x10, 0x37]);
        cpu.write_rn_l(7, 0b0101_0101_0101_0101_0101_0101_0101_0101)
            .unwrap();
        let opcode = cpu.fetch();
        let state = cpu.exec(opcode).unwrap();
        assert_eq!(state, 2);
        assert_eq!(cpu.ccr, 0b00001000);
        assert_eq!(
            cpu.read_rn_l(7).unwrap(),
            0b1010_1010_1010_1010_1010_1010_1010_1010
        );

        // check CCR V
        let mut cpu = Cpu::new();
        cpu.ccr = 0b00001110;

        cpu.bus.memory[0..2].copy_from_slice(&[0x10, 0x30]);
        cpu.write_rn_l(0, 0b1010_1010_1010_1010_1010_1010_1010_1010)
            .unwrap();
        let opcode = cpu.fetch();
        let state = cpu.exec(opcode).unwrap();
        assert_eq!(state, 2);
        assert_eq!(cpu.ccr, 0b00000001);
        assert_eq!(
            cpu.read_rn_l(0).unwrap(),
            0b0101_0101_0101_0101_0101_0101_0101_0100
        );

        // check CCR C, Z
        let mut cpu = Cpu::new();
        cpu.ccr = 0b00001010;

        cpu.bus.memory[0..2].copy_from_slice(&[0x10, 0x30]);
        cpu.write_rn_l(0, 0b1000_0000_0000_0000_0000_0000_0000_0000)
            .unwrap();
        let opcode = cpu.fetch();
        let state = cpu.exec(opcode).unwrap();
        assert_eq!(state, 2);
        assert_eq!(cpu.ccr, 0b00000101);
        assert_eq!(cpu.read_rn_l(0).unwrap(), 0);
    }
}
