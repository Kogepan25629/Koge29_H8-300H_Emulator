use crate::cpu::{Cpu, CCR};
use anyhow::{bail, Context as _, Result};

impl Cpu {
    pub(in super::super) async fn add_w(&mut self, opcode: u16) -> Result<usize> {
        match (opcode >> 8) as u8 {
            0x79 => return self.add_w_imm(opcode).await,
            0x09 => return self.add_w_rn(opcode),
            _ => bail!("invalid opcode [{:>04x}]", opcode),
        }
    }

    fn add_w_proc(&mut self, dest: u16, src: u16) -> u16 {
        let (value, overflowed) = (dest as i16).overflowing_add(src as i16);
        if (dest & 0x0fff) + (src & 0x0fff) > 0x0fff {
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

        if (dest as u32) + (src as u32) > 0xffff {
            self.write_ccr(CCR::C, 1);
        } else {
            self.write_ccr(CCR::C, 0);
        }

        value as u16
    }

    async fn add_w_imm(&mut self, opcode: u16) -> Result<usize> {
        let imm = self.fetch().await;
        let mut f = || -> Result<usize> {
            let register = Cpu::get_nibble_opcode(opcode, 4)?;
            let dest = self.read_rn_w(register)?;
            let result = self.add_w_proc(dest, imm);
            self.write_rn_w(register, result)?;
            Ok(4)
        };
        f().with_context(|| format!("imm(opcode2) [{:x}]", imm))
    }

    fn add_w_rn(&mut self, opcode: u16) -> Result<usize> {
        let mut f = || -> Result<usize> {
            let register_dest = Cpu::get_nibble_opcode(opcode, 4)?;
            let dest = self.read_rn_w(register_dest)?;
            let register_src = Cpu::get_nibble_opcode(opcode, 3)? & 0x7;
            let src = self.read_rn_w(register_src)?;
            let result = self.add_w_proc(dest, src);
            self.write_rn_w(register_dest, result)?;
            Ok(2)
        };
        f()
    }
}
