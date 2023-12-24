use crate::cpu::{Cpu, StateType};
use anyhow::{bail, Result};

impl Cpu {
    pub(in super::super) async fn bclr_rn_from_imm(&mut self, opcode: u16) -> Result<u8> {
        let register = Cpu::get_nibble_opcode(opcode, 4)?;
        let value = self.read_rn_b(register)?;
        let imm = Cpu::get_nibble_opcode(opcode, 3)? & 7;
        self.write_rn_b(register, value & !(1 << imm))?;
        Ok(self.calc_state(StateType::I, 1).await?)
    }

    pub(in super::super) async fn bclr_rn_from_rn(&mut self, opcode: u16) -> Result<u8> {
        let register_bit = Cpu::get_nibble_opcode(opcode, 3)?;
        let register_value = Cpu::get_nibble_opcode(opcode, 4)?;
        let bit = self.read_rn_b(register_bit)? & 7;
        let value = self.read_rn_b(register_value)?;
        self.write_rn_b(register_value, value & !(1 << bit))?;
        Ok(self.calc_state(StateType::I, 1).await?)
    }

    pub(in super::super) async fn bclr_ern(&mut self, opcode: u16, opcode2: u16) -> Result<u8> {
        let register_ern = Cpu::get_nibble_opcode(opcode, 3)?;
        let access_addr = self.get_addr_ern(register_ern)?;
        match opcode2 & 0xff0f {
            0x7200 => {
                let value = self.read_ern_b(register_ern).await?;
                let imm = Cpu::get_nibble_opcode(opcode2, 3)? & 7;
                self.write_ern_b(register_ern, value & !(1 << imm)).await?;
            }
            0x6200 => {
                let register_bit = Cpu::get_nibble_opcode(opcode2, 3)?;
                let bit = self.read_rn_b(register_bit)? & 7;
                let value = self.read_ern_b(register_ern).await?;
                self.write_ern_b(register_ern, value & !(1 << bit)).await?;
            }
            _ => bail!("invalid opcode [{:>04x}]", opcode),
        }

        return Ok(self.calc_state(StateType::I, 2).await?
            + self
                .calc_state_with_addr(StateType::L, 2, access_addr)
                .await?);
    }

    pub(in super::super) async fn bclr_abs(&mut self, opcode: u16, opcode2: u16) -> Result<u8> {
        let access_addr = self.get_addr_abs8(opcode as u8);
        match opcode2 & 0xff0f {
            0x7200 => {
                let imm = Cpu::get_nibble_opcode(opcode2, 3)? & 7;
                let value = self.read_abs8_b(opcode as u8).await?;
                self.write_abs8_b(opcode as u8, value & !(1 << imm)).await?;
            }
            0x6200 => {
                let register = Cpu::get_nibble_opcode(opcode2, 3)?;
                let bit = self.read_rn_b(register)? & 7;
                let value = self.read_abs8_b(opcode as u8).await?;
                self.write_abs8_b(opcode as u8, value & !(1 << bit)).await?;
            }
            _ => bail!("invalid opcode [{:>04x}]", opcode),
        }

        return Ok(self.calc_state(StateType::I, 2).await?
            + self
                .calc_state_with_addr(StateType::L, 2, access_addr)
                .await?);
    }
}

#[cfg(test)]
mod tests {
    use crate::{cpu::Cpu, memory::MEMORY_START_ADDR};

    #[tokio::test]
    async fn test_bclr_rn_from_imm() {
        // bit 0
        let mut cpu = Cpu::new();
        cpu.pc = MEMORY_START_ADDR;
        cpu.write_rn_b(0, 0xff).unwrap();
        cpu.bus.lock().await.memory[0..2].copy_from_slice(&[0x72, 0x00]);
        let opcode = cpu.fetch().await;
        let state = cpu.exec(opcode).await.unwrap();
        assert_eq!(state, 2);
        assert_eq!(cpu.read_rn_b(0).unwrap(), 0xfe);

        // bit 7
        let mut cpu = Cpu::new();
        cpu.pc = MEMORY_START_ADDR;
        cpu.write_rn_b(0, 0xff).unwrap();
        cpu.bus.lock().await.memory[0..2].copy_from_slice(&[0x72, 0x70]);
        let opcode = cpu.fetch().await;
        let state = cpu.exec(opcode).await.unwrap();
        assert_eq!(state, 2);
        assert_eq!(cpu.read_rn_b(0).unwrap(), 0x7f);

        // register 0xf
        let mut cpu = Cpu::new();
        cpu.pc = MEMORY_START_ADDR;
        cpu.write_rn_b(0xf, 0xff).unwrap();
        cpu.bus.lock().await.memory[0..2].copy_from_slice(&[0x72, 0x0f]);
        let opcode = cpu.fetch().await;
        let state = cpu.exec(opcode).await.unwrap();
        assert_eq!(state, 2);
        assert_eq!(cpu.read_rn_b(0xf).unwrap(), 0xfe);
    }

    #[tokio::test]
    async fn test_bclr_rn_from_rn() {
        // bit 0
        let mut cpu = Cpu::new();
        cpu.pc = MEMORY_START_ADDR;
        cpu.write_rn_b(0, 0).unwrap();
        cpu.write_rn_b(0xf, 0xff).unwrap();
        cpu.bus.lock().await.memory[0..2].copy_from_slice(&[0x62, 0x0f]);
        let opcode = cpu.fetch().await;
        let state = cpu.exec(opcode).await.unwrap();
        assert_eq!(state, 2);
        assert_eq!(cpu.read_rn_b(0xf).unwrap(), 0xfe);

        // bit 7
        let mut cpu = Cpu::new();
        cpu.pc = MEMORY_START_ADDR;
        cpu.write_rn_b(0, 7).unwrap();
        cpu.write_rn_b(0xf, 0xff).unwrap();
        cpu.bus.lock().await.memory[0..2].copy_from_slice(&[0x62, 0x0f]);
        let opcode = cpu.fetch().await;
        let state = cpu.exec(opcode).await.unwrap();
        assert_eq!(state, 2);
        assert_eq!(cpu.read_rn_b(0xf).unwrap(), 0x7f);

        // register 0xf
        let mut cpu = Cpu::new();
        cpu.pc = MEMORY_START_ADDR;
        cpu.write_rn_b(0xf, 0).unwrap();
        cpu.write_rn_b(0, 0xff).unwrap();
        cpu.bus.lock().await.memory[0..2].copy_from_slice(&[0x62, 0xf0]);
        let opcode = cpu.fetch().await;
        let state = cpu.exec(opcode).await.unwrap();
        assert_eq!(state, 2);
        assert_eq!(cpu.read_rn_b(0).unwrap(), 0xfe);
    }

    #[tokio::test]
    async fn test_bclr_ern() {
        ////////
        // imm

        // bit 0
        let mut cpu = Cpu::new();
        cpu.pc = MEMORY_START_ADDR;
        cpu.write_abs24_b(0xffcf20, 0xff).await.unwrap();
        cpu.write_rn_l(0, 0xffcf20).unwrap();
        cpu.bus.lock().await.memory[0..4].copy_from_slice(&[0x7d, 0x00, 0x72, 0x00]);
        let opcode = cpu.fetch().await;
        let state = cpu.exec(opcode).await.unwrap();
        assert_eq!(state, 8);
        assert_eq!(cpu.read_ern_b(0).await.unwrap(), 0xfe);

        // bit 7
        let mut cpu = Cpu::new();
        cpu.pc = MEMORY_START_ADDR;
        cpu.write_abs24_b(0xffcf20, 0xff).await.unwrap();
        cpu.write_rn_l(0, 0xffcf20).unwrap();
        cpu.bus.lock().await.memory[0..4].copy_from_slice(&[0x7d, 0x00, 0x72, 0x70]);
        let opcode = cpu.fetch().await;
        let state = cpu.exec(opcode).await.unwrap();
        assert_eq!(state, 8);
        assert_eq!(cpu.read_ern_b(0).await.unwrap(), 0x7f);

        // register 7
        let mut cpu = Cpu::new();
        cpu.pc = MEMORY_START_ADDR;
        cpu.write_abs24_b(0xffcf20, 0xff).await.unwrap();
        cpu.write_rn_l(7, 0xffcf20).unwrap();
        cpu.bus.lock().await.memory[0..4].copy_from_slice(&[0x7d, 0x70, 0x72, 0x70]);
        let opcode = cpu.fetch().await;
        let state = cpu.exec(opcode).await.unwrap();
        assert_eq!(state, 8);
        assert_eq!(cpu.read_ern_b(7).await.unwrap(), 0x7f);

        ////////
        // rn

        // bit 0
        let mut cpu = Cpu::new();
        cpu.pc = MEMORY_START_ADDR;
        cpu.write_abs24_b(0xffcf20, 0xff).await.unwrap();
        cpu.write_rn_l(0, 0xffcf20).unwrap();
        cpu.write_rn_b(0xf, 0).unwrap();
        cpu.bus.lock().await.memory[0..4].copy_from_slice(&[0x7d, 0x00, 0x62, 0xf0]);
        let opcode = cpu.fetch().await;
        let state = cpu.exec(opcode).await.unwrap();
        assert_eq!(state, 8);
        assert_eq!(cpu.read_ern_b(0).await.unwrap(), 0xfe);

        // bit 7
        let mut cpu = Cpu::new();
        cpu.pc = MEMORY_START_ADDR;
        cpu.write_abs24_b(0xffcf20, 0xff).await.unwrap();
        cpu.write_rn_l(0, 0xffcf20).unwrap();
        cpu.write_rn_b(0xf, 7).unwrap();
        cpu.bus.lock().await.memory[0..4].copy_from_slice(&[0x7d, 0x00, 0x62, 0xf0]);
        let opcode = cpu.fetch().await;
        let state = cpu.exec(opcode).await.unwrap();
        assert_eq!(state, 8);
        assert_eq!(cpu.read_ern_b(0).await.unwrap(), 0x7f);

        // register 7
        let mut cpu = Cpu::new();
        cpu.pc = MEMORY_START_ADDR;
        cpu.write_abs24_b(0xffcf20, 0xff).await.unwrap();
        cpu.write_rn_l(7, 0xffcf20).unwrap();
        cpu.write_rn_b(0, 7).unwrap();
        cpu.bus.lock().await.memory[0..4].copy_from_slice(&[0x7d, 0x70, 0x62, 0x00]);
        let opcode = cpu.fetch().await;
        let state = cpu.exec(opcode).await.unwrap();
        assert_eq!(state, 8);
        assert_eq!(cpu.read_ern_b(7).await.unwrap(), 0x7f);
    }

    #[tokio::test]
    async fn test_bclr_abs() {
        ////////
        // imm

        // bit 0
        let mut cpu = Cpu::new();
        cpu.pc = MEMORY_START_ADDR;
        cpu.write_abs24_b(0xffff12, 0xff).await.unwrap();
        cpu.bus.lock().await.memory[0..4].copy_from_slice(&[0x7f, 0x12, 0x72, 0x00]);
        let opcode = cpu.fetch().await;
        let state = cpu.exec(opcode).await.unwrap();
        assert_eq!(state, 8);
        assert_eq!(cpu.read_abs24_b(0xffff12).await.unwrap(), 0xfe);

        // bit 7
        let mut cpu = Cpu::new();
        cpu.pc = MEMORY_START_ADDR;
        cpu.write_abs24_b(0xffff12, 0xff).await.unwrap();
        cpu.bus.lock().await.memory[0..4].copy_from_slice(&[0x7f, 0x12, 0x72, 0x70]);
        let opcode = cpu.fetch().await;
        let state = cpu.exec(opcode).await.unwrap();
        assert_eq!(state, 8);
        assert_eq!(cpu.read_abs24_b(0xffff12).await.unwrap(), 0x7f);

        ////////
        // rn

        // bit 0
        let mut cpu = Cpu::new();
        cpu.pc = MEMORY_START_ADDR;
        cpu.write_abs24_b(0xffff12, 0xff).await.unwrap();
        cpu.write_rn_b(0, 0).unwrap();
        cpu.bus.lock().await.memory[0..4].copy_from_slice(&[0x7f, 0x12, 0x62, 0x00]);
        let opcode = cpu.fetch().await;
        let state = cpu.exec(opcode).await.unwrap();
        assert_eq!(state, 8);
        assert_eq!(cpu.read_abs24_b(0xffff12).await.unwrap(), 0xfe);

        // bit 7
        let mut cpu = Cpu::new();
        cpu.pc = MEMORY_START_ADDR;
        cpu.write_abs24_b(0xffff12, 0xff).await.unwrap();
        cpu.write_rn_b(0, 7).unwrap();
        cpu.bus.lock().await.memory[0..4].copy_from_slice(&[0x7f, 0x12, 0x62, 0x00]);
        let opcode = cpu.fetch().await;
        let state = cpu.exec(opcode).await.unwrap();
        assert_eq!(state, 8);
        assert_eq!(cpu.read_abs24_b(0xffff12).await.unwrap(), 0x7f);

        // register 0xf
        let mut cpu = Cpu::new();
        cpu.pc = MEMORY_START_ADDR;
        cpu.write_abs24_b(0xffff12, 0xff).await.unwrap();
        cpu.write_rn_b(0xf, 7).unwrap();
        cpu.bus.lock().await.memory[0..4].copy_from_slice(&[0x7f, 0x12, 0x62, 0xf0]);
        let opcode = cpu.fetch().await;
        let state = cpu.exec(opcode).await.unwrap();
        assert_eq!(state, 8);
        assert_eq!(cpu.read_abs24_b(0xffff12).await.unwrap(), 0x7f);
    }
}
