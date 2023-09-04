use crate::cpu::{Cpu, CCR};
use anyhow::{bail, Result};

impl Cpu {
    pub(in super::super) async fn cmp_w(&mut self, opcode: u16) -> Result<usize> {
        match (opcode >> 8) as u8 {
            0x79 => return self.cmp_w_imm(opcode).await,
            0x1d => return self.cmp_w_rn(opcode),
            _ => bail!("invalid opcode [{:>04x}]", opcode),
        }
    }

    fn cmp_w_proc(&mut self, dest: u16, src: u16) -> u16 {
        let (value, overflowed) = (dest as i16).overflowing_sub(src as i16);
        if (dest & 0x0fff) + (!src & 0x0fff) + 1 > 0x0fff {
            self.write_ccr(CCR::H, 1);
        } else {
            self.write_ccr(CCR::H, 0);
        }

        if value < 0 {
            self.write_ccr(CCR::N, 1);
        } else {
            self.write_ccr(CCR::N, 0);
        }

        if value == 0 {
            self.write_ccr(CCR::Z, 1);
        } else {
            self.write_ccr(CCR::Z, 0);
        }

        if overflowed {
            self.write_ccr(CCR::V, 1);
        } else {
            self.write_ccr(CCR::V, 0);
        }

        if (dest as u32) + (!src as u32) + 1 > 0xffff {
            self.write_ccr(CCR::C, 1);
        } else {
            self.write_ccr(CCR::C, 0);
        }

        value as u16
    }

    async fn cmp_w_imm(&mut self, opcode: u16) -> Result<usize> {
        let imm = self.fetch().await;
        let dest = self.read_rn_w(Cpu::get_nibble_opcode(opcode, 4)?)?;
        self.cmp_w_proc(dest, imm);
        Ok(4)
    }

    fn cmp_w_rn(&mut self, opcode: u16) -> Result<usize> {
        let src = self.read_rn_w(Cpu::get_nibble_opcode(opcode, 3)? & 0x7)?;
        let dest = self.read_rn_w(Cpu::get_nibble_opcode(opcode, 4)?)?;
        self.cmp_w_proc(dest, src);
        Ok(2)
    }
}
