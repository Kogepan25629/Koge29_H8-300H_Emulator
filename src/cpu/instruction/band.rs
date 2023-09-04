use crate::cpu::{Cpu, CCR};
use anyhow::Result;

impl Cpu {
    pub(in super::super) fn band_rn(&mut self, opcode: u16) -> Result<usize> {
        let register = Cpu::get_nibble_opcode(opcode, 4)?;
        let value = self.read_rn_b(register)?;
        let imm = Cpu::get_nibble_opcode(opcode, 3)? & 7;
        self.write_ccr(CCR::C, (value >> imm) & self.read_ccr(CCR::C));
        Ok(2)
    }

    pub(in super::super) async fn band_ern(&mut self, opcode: u16, opcode2: u16) -> Result<usize> {
        let register = Cpu::get_nibble_opcode(opcode, 3)?;
        let value = self.read_ern_b(register).await?;
        let imm = Cpu::get_nibble_opcode(opcode2, 3)? & 7;
        self.write_ccr(CCR::C, (value >> imm) & self.read_ccr(CCR::C));
        return Ok(6);
    }

    pub(in super::super) async fn band_abs(&mut self, opcode: u16, opcode2: u16) -> Result<usize> {
        let imm = Cpu::get_nibble_opcode(opcode2, 3)? & 7;
        let value = self.read_abs8_b(opcode as u8).await?;
        self.write_ccr(CCR::C, (value >> imm) & self.read_ccr(CCR::C));
        return Ok(6);
    }
}

#[cfg(test)]
mod tests {
    use crate::cpu::Cpu;

    #[tokio::test]
    async fn test_band_rn() {
        let mut cpu = Cpu::new();
        cpu.ccr = 1;
        cpu.write_rn_b(0, 0x01).unwrap();
        cpu.bus.lock().await.memory[0..2].copy_from_slice(&[0x76, 0x00]);
        let opcode = cpu.fetch().await;
        let state = cpu.exec(opcode).await.unwrap();
        assert_eq!(state, 2);
        assert_eq!(cpu.ccr, 1);

        // bit7
        let mut cpu = Cpu::new();
        cpu.ccr = 1;
        cpu.write_rn_b(0, 0x80).unwrap();
        cpu.bus.lock().await.memory[0..2].copy_from_slice(&[0x76, 0x70]);
        let opcode = cpu.fetch().await;
        let state = cpu.exec(opcode).await.unwrap();
        assert_eq!(state, 2);
        assert_eq!(cpu.ccr, 1);

        // register 0xf
        let mut cpu = Cpu::new();
        cpu.ccr = 1;
        cpu.write_rn_b(0xf, 0x01).unwrap();
        cpu.bus.lock().await.memory[0..2].copy_from_slice(&[0x76, 0x0f]);
        let opcode = cpu.fetch().await;
        let state = cpu.exec(opcode).await.unwrap();
        assert_eq!(state, 2);
        assert_eq!(cpu.ccr, 1);

        let mut cpu = Cpu::new();
        cpu.ccr = 1;
        cpu.write_rn_b(0, 0xfe).unwrap();
        cpu.bus.lock().await.memory[0..2].copy_from_slice(&[0x76, 0x00]);
        let opcode = cpu.fetch().await;
        let state = cpu.exec(opcode).await.unwrap();
        assert_eq!(state, 2);
        assert_eq!(cpu.ccr, 0);

        let mut cpu = Cpu::new();
        cpu.ccr = 0;
        cpu.write_rn_b(0, 0x01).unwrap();
        cpu.bus.lock().await.memory[0..2].copy_from_slice(&[0x76, 0x00]);
        let opcode = cpu.fetch().await;
        let state = cpu.exec(opcode).await.unwrap();
        assert_eq!(state, 2);
        assert_eq!(cpu.ccr, 0);
    }

    #[tokio::test]
    async fn test_band_ern() {
        let mut cpu = Cpu::new();
        cpu.ccr = 1;
        cpu.write_abs24_b(0xffcf20, 0x01).await.unwrap();
        cpu.write_rn_l(0, 0xffcf20).unwrap();
        cpu.bus.lock().await.memory[0..4].copy_from_slice(&[0x7c, 0x00, 0x76, 0x00]);
        let opcode = cpu.fetch().await;
        let state = cpu.exec(opcode).await.unwrap();
        assert_eq!(state, 6);
        assert_eq!(cpu.ccr, 1);

        // bit 7
        let mut cpu = Cpu::new();
        cpu.ccr = 1;
        cpu.write_abs24_b(0xffcf20, 0x80).await.unwrap();
        cpu.write_rn_l(0, 0xffcf20).unwrap();
        cpu.bus.lock().await.memory[0..4].copy_from_slice(&[0x7c, 0x00, 0x76, 0x70]);
        let opcode = cpu.fetch().await;
        let state = cpu.exec(opcode).await.unwrap();
        assert_eq!(state, 6);
        assert_eq!(cpu.ccr, 1);

        // register 7
        let mut cpu = Cpu::new();
        cpu.ccr = 1;
        cpu.write_abs24_b(0xffcf20, 0x01).await.unwrap();
        cpu.write_rn_l(7, 0xffcf20).unwrap();
        cpu.bus.lock().await.memory[0..4].copy_from_slice(&[0x7c, 0x70, 0x76, 0x00]);
        let opcode = cpu.fetch().await;
        let state = cpu.exec(opcode).await.unwrap();
        assert_eq!(state, 6);
        assert_eq!(cpu.ccr, 1);

        let mut cpu = Cpu::new();
        cpu.ccr = 1;
        cpu.write_abs24_b(0xffcf20, 0xfe).await.unwrap();
        cpu.write_rn_l(0, 0xffcf20).unwrap();
        cpu.bus.lock().await.memory[0..4].copy_from_slice(&[0x7c, 0x00, 0x76, 0x00]);
        let opcode = cpu.fetch().await;
        let state = cpu.exec(opcode).await.unwrap();
        assert_eq!(state, 6);
        assert_eq!(cpu.ccr, 0);

        let mut cpu = Cpu::new();
        cpu.ccr = 0;
        cpu.write_abs24_b(0xffcf20, 0x01).await.unwrap();
        cpu.write_rn_l(0, 0xffcf20).unwrap();
        cpu.bus.lock().await.memory[0..4].copy_from_slice(&[0x7c, 0x00, 0x76, 0x00]);
        let opcode = cpu.fetch().await;
        let state = cpu.exec(opcode).await.unwrap();
        assert_eq!(state, 6);
        assert_eq!(cpu.ccr, 0);
    }

    #[tokio::test]
    async fn test_band_abs() {
        let mut cpu = Cpu::new();
        cpu.ccr = 1;
        cpu.write_abs24_b(0xffff12, 0x01).await.unwrap();
        cpu.bus.lock().await.memory[0..4].copy_from_slice(&[0x7e, 0x12, 0x76, 0x00]);
        let opcode = cpu.fetch().await;
        let state = cpu.exec(opcode).await.unwrap();
        assert_eq!(state, 6);
        assert_eq!(cpu.ccr, 1);

        // bit 7
        let mut cpu = Cpu::new();
        cpu.ccr = 1;
        cpu.write_abs24_b(0xffff12, 0x80).await.unwrap();
        cpu.bus.lock().await.memory[0..4].copy_from_slice(&[0x7e, 0x12, 0x76, 0x70]);
        let opcode = cpu.fetch().await;
        let state = cpu.exec(opcode).await.unwrap();
        assert_eq!(state, 6);
        assert_eq!(cpu.ccr, 1);

        let mut cpu = Cpu::new();
        cpu.ccr = 1;
        cpu.write_abs24_b(0xffff12, 0xfe).await.unwrap();
        cpu.bus.lock().await.memory[0..4].copy_from_slice(&[0x7e, 0x12, 0x76, 0x00]);
        let opcode = cpu.fetch().await;
        let state = cpu.exec(opcode).await.unwrap();
        assert_eq!(state, 6);
        assert_eq!(cpu.ccr, 0);

        let mut cpu = Cpu::new();
        cpu.ccr = 0;
        cpu.write_abs24_b(0xffff12, 0x01).await.unwrap();
        cpu.bus.lock().await.memory[0..4].copy_from_slice(&[0x7e, 0x12, 0x76, 0x00]);
        let opcode = cpu.fetch().await;
        let state = cpu.exec(opcode).await.unwrap();
        assert_eq!(state, 6);
        assert_eq!(cpu.ccr, 0);
    }
}
