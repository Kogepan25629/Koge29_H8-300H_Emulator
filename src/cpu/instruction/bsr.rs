use crate::cpu::{Cpu, StateType};
use anyhow::Result;

impl Cpu {
    pub(in super::super) async fn bsr_disp16(&mut self, opcode: u16) -> Result<u8> {
        let access_addr = (self.read_rn_l(7)? - 4) & 0x00ffffff;
        self.write_dec_ern_l(7, self.pc).await?;
        let disp = ((opcode as u8) as i8) as i32;
        self.pc = (self.pc as i32 + disp) as u32;
        Ok(self.calc_state(StateType::I, 2).await?
            + self
                .calc_state_with_addr(StateType::K, 1, access_addr)
                .await?
            + self.calc_state(StateType::N, 2).await?)
    }

    pub(in super::super) async fn bsr_disp24(&mut self, _opcode: u16) -> Result<u8> {
        let access_addr = (self.read_rn_l(7)? - 4) & 0x00ffffff;
        let opcode2 = self.fetch().await;
        self.write_dec_ern_l(7, self.pc).await?;
        let disp = (opcode2 as i16) as i32;
        self.pc = (self.pc as i32 + disp) as u32;
        Ok(self.calc_state(StateType::I, 2).await?
            + self
                .calc_state_with_addr(StateType::K, 2, access_addr)
                .await?
            + self.calc_state(StateType::N, 2).await?)
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        cpu::Cpu,
        memory::{MEMORY_END_ADDR, MEMORY_START_ADDR},
    };

    #[tokio::test]
    async fn test_bsr_disp16() {
        let mut cpu = Cpu::new();
        cpu.er[7] = MEMORY_END_ADDR - 0xf;
        cpu.bus.lock().await.memory[0..2].copy_from_slice(&[0x55, 0x15]);
        let opcode = cpu.fetch().await;
        let state = cpu.exec(opcode).await.unwrap();
        assert_eq!(state, 8);
        assert_eq!(cpu.ccr, 0);
        assert_eq!(cpu.read_ern_l(7).await.unwrap(), MEMORY_START_ADDR + 2);
        assert_eq!(cpu.pc, MEMORY_START_ADDR + 2 + 0x15);

        let mut cpu = Cpu::new();
        cpu.er[7] = MEMORY_END_ADDR - 0xf;
        cpu.pc = MEMORY_START_ADDR + 100;
        cpu.bus.lock().await.memory[100..102].copy_from_slice(&[0x55, 0xf6]);
        let opcode = cpu.fetch().await;
        let state = cpu.exec(opcode).await.unwrap();
        assert_eq!(state, 8);
        assert_eq!(cpu.ccr, 0);
        assert_eq!(cpu.read_ern_l(7).await.unwrap(), MEMORY_START_ADDR + 102);
        assert_eq!(cpu.pc, MEMORY_START_ADDR + 102 - 10);
    }

    #[tokio::test]
    async fn test_bsr_disp24() {
        let mut cpu = Cpu::new();
        cpu.er[7] = MEMORY_END_ADDR - 0xf;
        cpu.bus.lock().await.memory[0..4].copy_from_slice(&[0x5c, 0x00, 0x00, 0x15]);
        let opcode = cpu.fetch().await;
        let state = cpu.exec(opcode).await.unwrap();
        assert_eq!(state, 10);
        assert_eq!(cpu.ccr, 0);
        assert_eq!(cpu.read_ern_l(7).await.unwrap(), MEMORY_START_ADDR + 4);
        assert_eq!(cpu.pc, MEMORY_START_ADDR + 4 + 0x15);

        let mut cpu = Cpu::new();
        cpu.er[7] = MEMORY_END_ADDR - 0xf;
        cpu.pc = MEMORY_START_ADDR + 100;
        cpu.bus.lock().await.memory[100..104].copy_from_slice(&[0x5c, 0x00, 0xff, 0xf6]);
        let opcode = cpu.fetch().await;
        let state = cpu.exec(opcode).await.unwrap();
        assert_eq!(state, 10);
        assert_eq!(cpu.ccr, 0);
        assert_eq!(cpu.read_ern_l(7).await.unwrap(), MEMORY_START_ADDR + 104);
        assert_eq!(cpu.pc, MEMORY_START_ADDR + 104 - 10);
    }
}
