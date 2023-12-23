use crate::cpu::{Cpu, StateType};
use anyhow::Result;

impl Cpu {
    pub(in super::super) async fn bst_rn(&mut self, opcode: u16) -> Result<u8> {
        let register = Cpu::get_nibble_opcode(opcode, 4)?;
        let value = self.read_rn_b(register)?;
        let imm = Cpu::get_nibble_opcode(opcode, 3)? & 7;
        let c = self.ccr & 1;
        if c == 1 {
            self.write_rn_b(register, value | (c << imm))?;
        } else {
            self.write_rn_b(register, value & !(c << imm))?;
        }
        Ok(self.calc_state(StateType::I, 1).await?)
    }

    pub(in super::super) async fn bst_ern(&mut self, opcode: u16, opcode2: u16) -> Result<u8> {
        let register = Cpu::get_nibble_opcode(opcode, 3)?;
        let access_addr = self.get_addr_ern(register)?;
        let value = self.read_ern_b(register).await?;
        let imm = Cpu::get_nibble_opcode(opcode2, 3)? & 7;
        let c = self.ccr & 1;
        if c == 1 {
            self.write_ern_b(register, value | (c << imm)).await?;
        } else {
            self.write_ern_b(register, value & !(c << imm)).await?;
        }
        Ok(self.calc_state(StateType::I, 2).await?
            + self
                .calc_state_with_addr(StateType::L, 2, access_addr)
                .await?)
    }

    pub(in super::super) async fn bst_abs(&mut self, opcode: u16, opcode2: u16) -> Result<u8> {
        let imm = Cpu::get_nibble_opcode(opcode2, 3)? & 7;
        let value = self.read_abs8_b(opcode as u8).await?;
        let c = self.ccr & 1;
        if c == 1 {
            self.write_abs8_b(opcode as u8, value | (c << imm)).await?;
        } else {
            self.write_abs8_b(opcode as u8, value & !(c << imm)).await?;
        }
        let access_addr = self.get_addr_abs8(opcode as u8);
        Ok(self.calc_state(StateType::I, 2).await?
            + self
                .calc_state_with_addr(StateType::L, 2, access_addr)
                .await?)
    }
}

#[cfg(test)]
mod tests {
    use crate::cpu::Cpu;

    #[tokio::test]
    async fn test_bst_rn() {
        let mut cpu = Cpu::new();
        cpu.ccr = 1;
        cpu.write_rn_b(0, 0xfe).unwrap();
        cpu.bus.lock().await.memory[0..2].copy_from_slice(&[0x67, 0x00]);
        let opcode = cpu.fetch().await;
        let state = cpu.exec(opcode).await.unwrap();
        assert_eq!(state, 2);
        assert_eq!(cpu.ccr, 1);
        assert_eq!(cpu.read_rn_b(0).unwrap(), 0xff);

        // bit7
        let mut cpu = Cpu::new();
        cpu.ccr = 1;
        cpu.write_rn_b(0, 0x7f).unwrap();
        cpu.bus.lock().await.memory[0..2].copy_from_slice(&[0x67, 0x70]);
        let opcode = cpu.fetch().await;
        let state = cpu.exec(opcode).await.unwrap();
        assert_eq!(state, 2);
        assert_eq!(cpu.ccr, 1);
        assert_eq!(cpu.read_rn_b(0).unwrap(), 0xff);

        // register 0xf
        let mut cpu = Cpu::new();
        cpu.ccr = 1;
        cpu.write_rn_b(0xf, 0xfe).unwrap();
        cpu.bus.lock().await.memory[0..2].copy_from_slice(&[0x67, 0x0f]);
        let opcode = cpu.fetch().await;
        let state = cpu.exec(opcode).await.unwrap();
        assert_eq!(state, 2);
        assert_eq!(cpu.ccr, 1);
        assert_eq!(cpu.read_rn_b(0xf).unwrap(), 0xff);

        let mut cpu = Cpu::new();
        cpu.ccr = 0;
        cpu.write_rn_b(0, 0xfe).unwrap();
        cpu.bus.lock().await.memory[0..2].copy_from_slice(&[0x67, 0x00]);
        let opcode = cpu.fetch().await;
        let state = cpu.exec(opcode).await.unwrap();
        assert_eq!(state, 2);
        assert_eq!(cpu.ccr, 0);
        assert_eq!(cpu.read_rn_b(0).unwrap(), 0xfe);
    }

    #[tokio::test]
    async fn test_bst_ern() {
        let mut cpu = Cpu::new();
        cpu.ccr = 1;
        cpu.write_abs24_b(0xffcf20, 0xfe).await.unwrap();
        cpu.write_rn_l(0, 0xffcf20).unwrap();
        cpu.bus.lock().await.memory[0..4].copy_from_slice(&[0x7d, 0x00, 0x67, 0x00]);
        let opcode = cpu.fetch().await;
        let state = cpu.exec(opcode).await.unwrap();
        assert_eq!(state, 8);
        assert_eq!(cpu.ccr, 1);
        assert_eq!(cpu.read_ern_b(0).await.unwrap(), 0xff);

        // bit 7
        let mut cpu = Cpu::new();
        cpu.ccr = 1;
        cpu.write_abs24_b(0xffcf20, 0x7f).await.unwrap();
        cpu.write_rn_l(0, 0xffcf20).unwrap();
        cpu.bus.lock().await.memory[0..4].copy_from_slice(&[0x7d, 0x00, 0x67, 0x70]);
        let opcode = cpu.fetch().await;
        let state = cpu.exec(opcode).await.unwrap();
        assert_eq!(state, 8);
        assert_eq!(cpu.ccr, 1);
        assert_eq!(cpu.read_ern_b(0).await.unwrap(), 0xff);

        // register 7
        let mut cpu = Cpu::new();
        cpu.ccr = 1;
        cpu.write_abs24_b(0xffcf20, 0xfe).await.unwrap();
        cpu.write_rn_l(7, 0xffcf20).unwrap();
        cpu.bus.lock().await.memory[0..4].copy_from_slice(&[0x7d, 0x70, 0x67, 0x00]);
        let opcode = cpu.fetch().await;
        let state = cpu.exec(opcode).await.unwrap();
        assert_eq!(state, 8);
        assert_eq!(cpu.ccr, 1);
        assert_eq!(cpu.read_ern_b(7).await.unwrap(), 0xff);

        let mut cpu = Cpu::new();
        cpu.ccr = 0;
        cpu.write_abs24_b(0xffcf20, 0xfe).await.unwrap();
        cpu.write_rn_l(0, 0xffcf20).unwrap();
        cpu.bus.lock().await.memory[0..4].copy_from_slice(&[0x7d, 0x00, 0x67, 0x00]);
        let opcode = cpu.fetch().await;
        let state = cpu.exec(opcode).await.unwrap();
        assert_eq!(state, 8);
        assert_eq!(cpu.ccr, 0);
        assert_eq!(cpu.read_ern_b(0).await.unwrap(), 0xfe);
    }

    #[tokio::test]
    async fn test_bst_abs() {
        let mut cpu = Cpu::new();
        cpu.ccr = 1;
        cpu.write_abs24_b(0xffff12, 0xfe).await.unwrap();
        cpu.bus.lock().await.memory[0..4].copy_from_slice(&[0x7f, 0x12, 0x67, 0x00]);
        let opcode = cpu.fetch().await;
        let state = cpu.exec(opcode).await.unwrap();
        assert_eq!(state, 8);
        assert_eq!(cpu.ccr, 1);
        assert_eq!(cpu.read_abs24_b(0xffff12).await.unwrap(), 0xff);

        // bit 7
        let mut cpu = Cpu::new();
        cpu.ccr = 1;
        cpu.write_abs24_b(0xffff12, 0x7f).await.unwrap();
        cpu.bus.lock().await.memory[0..4].copy_from_slice(&[0x7f, 0x12, 0x67, 0x70]);
        let opcode = cpu.fetch().await;
        let state = cpu.exec(opcode).await.unwrap();
        assert_eq!(state, 8);
        assert_eq!(cpu.ccr, 1);
        assert_eq!(cpu.read_abs24_b(0xffff12).await.unwrap(), 0xff);

        let mut cpu = Cpu::new();
        cpu.ccr = 0;
        cpu.write_abs24_b(0xffff12, 0xfe).await.unwrap();
        cpu.bus.lock().await.memory[0..4].copy_from_slice(&[0x7f, 0x12, 0x67, 0x00]);
        let opcode = cpu.fetch().await;
        let state = cpu.exec(opcode).await.unwrap();
        assert_eq!(state, 8);
        assert_eq!(cpu.ccr, 0);
        assert_eq!(cpu.read_abs24_b(0xffff12).await.unwrap(), 0xfe);
    }
}
