use crate::cpu::Cpu;
use anyhow::{anyhow, Result};

impl Cpu {
    pub(in super::super) fn get_addr_disp16(addr: u32, disp: u16) -> Result<u32> {
        let result = addr
            .checked_add_signed((disp as i16) as i32)
            .ok_or_else(|| {
                anyhow!(
                    "attempt to add with overflow [{:x} + {:x}]",
                    addr,
                    disp as i16
                )
            })?
            & 0x00ffffff;
        Ok(result)
    }

    pub(in super::super) fn get_addr_disp24(addr: u32, disp: u32) -> Result<u32> {
        if disp & 0x800000 == 0x000000 {
            Ok((addr + disp) & 0x00ffffff)
        } else {
            let result = addr
                .checked_add_signed((0xff000000 + disp) as i32)
                .ok_or_else(|| {
                    anyhow!(
                        "attempt to add with overflow [{:x} + {:x}]",
                        addr,
                        (0xffff0000 + disp)
                    )
                })?
                & 0x00ffffff;
            Ok(result)
        }
    }

    pub(in super::super) async fn write_disp16_ern_b(
        &mut self,
        register_field: u8,
        disp: u16,
        value: u8,
    ) -> Result<()> {
        let addr = self.read_rn_l(register_field)?;
        self.write_abs24_b(Cpu::get_addr_disp16(addr, disp)?, value)
            .await?;
        Ok(())
    }

    pub(in super::super) async fn read_disp16_ern_b(
        &self,
        register_field: u8,
        disp: u16,
    ) -> Result<u8> {
        let addr = self.read_rn_l(register_field)?;
        Ok(self.read_abs24_b(Cpu::get_addr_disp16(addr, disp)?).await?)
    }

    pub(in super::super) async fn write_disp16_ern_w(
        &mut self,
        register_field: u8,
        disp: u16,
        value: u16,
    ) -> Result<()> {
        let addr = self.read_rn_l(register_field)?;
        self.write_abs24_w(Cpu::get_addr_disp16(addr, disp)?, value)
            .await?;
        Ok(())
    }

    pub(in super::super) async fn read_disp16_ern_w(
        &self,
        register_field: u8,
        disp: u16,
    ) -> Result<u16> {
        let addr = self.read_rn_l(register_field)?;
        Ok(self.read_abs24_w(Cpu::get_addr_disp16(addr, disp)?).await?)
    }

    pub(in super::super) async fn write_disp16_ern_l(
        &mut self,
        register_field: u8,
        disp: u16,
        value: u32,
    ) -> Result<()> {
        let addr = self.read_rn_l(register_field)?;
        self.write_abs24_l(Cpu::get_addr_disp16(addr, disp)?, value)
            .await?;
        Ok(())
    }

    pub(in super::super) async fn read_disp16_ern_l(
        &self,
        register_field: u8,
        disp: u16,
    ) -> Result<u32> {
        let addr = self.read_rn_l(register_field)?;
        Ok(self.read_abs24_l(Cpu::get_addr_disp16(addr, disp)?).await?)
    }

    ////
    pub(in super::super) async fn write_disp24_ern_b(
        &mut self,
        register_field: u8,
        disp: u32,
        value: u8,
    ) -> Result<()> {
        let addr = self.read_rn_l(register_field)?;
        self.write_abs24_b(Cpu::get_addr_disp24(addr, disp)?, value)
            .await?;
        Ok(())
    }

    pub(in super::super) async fn read_disp24_ern_b(
        &self,
        register_field: u8,
        disp: u32,
    ) -> Result<u8> {
        let addr = self.read_rn_l(register_field)?;
        Ok(self.read_abs24_b(Cpu::get_addr_disp24(addr, disp)?).await?)
    }

    pub(in super::super) async fn write_disp24_ern_w(
        &mut self,
        register_field: u8,
        disp: u32,
        value: u16,
    ) -> Result<()> {
        let addr = self.read_rn_l(register_field)?;
        self.write_abs24_w(Cpu::get_addr_disp24(addr, disp)?, value)
            .await?;
        Ok(())
    }

    pub(in super::super) async fn read_disp24_ern_w(
        &self,
        register_field: u8,
        disp: u32,
    ) -> Result<u16> {
        let addr = self.read_rn_l(register_field)?;
        Ok(self.read_abs24_w(Cpu::get_addr_disp24(addr, disp)?).await?)
    }

    pub(in super::super) async fn write_disp24_ern_l(
        &mut self,
        register_field: u8,
        disp: u32,
        value: u32,
    ) -> Result<()> {
        let addr = self.read_rn_l(register_field)?;
        self.write_abs24_l(Cpu::get_addr_disp24(addr, disp)?, value)
            .await?;
        Ok(())
    }

    pub(in super::super) async fn read_disp24_ern_l(
        &self,
        register_field: u8,
        disp: u32,
    ) -> Result<u32> {
        let addr = self.read_rn_l(register_field)?;
        Ok(self.read_abs24_l(Cpu::get_addr_disp24(addr, disp)?).await?)
    }
}

#[cfg(test)]
mod tests {
    use crate::cpu::Cpu;

    #[tokio::test]
    async fn test_write_disp16_ern_b() {
        let mut cpu = Cpu::new();
        cpu.er[0] = 0xfff000;
        cpu.write_disp16_ern_b(0, 0x0f10, 0xff).await.unwrap();
        assert_eq!(cpu.read_abs24_b(0xffff10).await.unwrap(), 0xff);

        cpu.er[1] = 0xfffe0a;
        cpu.write_disp16_ern_b(1, 0xfff6, 0xff).await.unwrap();
        assert_eq!(cpu.read_abs24_b(0xfffe00).await.unwrap(), 0xff);
    }

    #[tokio::test]
    async fn test_read_disp16_ern_b() {
        let mut cpu = Cpu::new();
        cpu.er[0] = 0xfff000;
        cpu.write_abs24_b(0xffff10, 0xff).await.unwrap();
        assert_eq!(cpu.read_disp16_ern_b(0, 0x0f10).await.unwrap(), 0xff);

        cpu.er[1] = 0xfffe0a;
        cpu.write_abs24_b(0xfffe00, 0xff).await.unwrap();
        assert_eq!(cpu.read_disp16_ern_b(1, 0xfff6).await.unwrap(), 0xff);
    }

    #[tokio::test]
    async fn test_write_disp16_ern_w() {
        let mut cpu = Cpu::new();
        cpu.er[0] = 0xfff000;
        cpu.write_disp16_ern_w(0, 0x0f10, 0x0fff).await.unwrap();
        assert_eq!(cpu.read_abs24_w(0xffff10).await.unwrap(), 0x0fff);

        cpu.er[1] = 0xfffe0a;
        cpu.write_disp16_ern_w(1, 0xfff6, 0x0fff).await.unwrap();
        assert_eq!(cpu.read_abs24_w(0xfffe00).await.unwrap(), 0x0fff);
    }

    #[tokio::test]
    async fn test_read_disp16_ern_w() {
        let mut cpu = Cpu::new();
        cpu.er[0] = 0xfff000;
        cpu.write_abs24_w(0xffff10, 0x0fff).await.unwrap();
        assert_eq!(cpu.read_disp16_ern_w(0, 0x0f10).await.unwrap(), 0x0fff);

        cpu.er[1] = 0xfffe0a;
        cpu.write_abs24_w(0xfffe00, 0x0fff).await.unwrap();
        assert_eq!(cpu.read_disp16_ern_w(1, 0xfff6).await.unwrap(), 0x0fff);
    }

    #[tokio::test]
    async fn test_write_disp16_ern_l() {
        let mut cpu = Cpu::new();
        cpu.er[0] = 0xfff000;
        cpu.write_disp16_ern_l(0, 0x0f10, 0x0f0fff0f).await.unwrap();
        assert_eq!(cpu.read_abs24_l(0xffff10).await.unwrap(), 0x0f0fff0f);

        cpu.er[1] = 0xfffe0a;
        cpu.write_disp16_ern_l(1, 0xfff6, 0x0f0fff0f).await.unwrap();
        assert_eq!(cpu.read_abs24_l(0xfffe00).await.unwrap(), 0x0f0fff0f);
    }

    #[tokio::test]
    async fn test_read_disp16_ern_l() {
        let mut cpu = Cpu::new();
        cpu.er[0] = 0xfff000;
        cpu.write_abs24_l(0xffff10, 0x0f0fff0f).await.unwrap();
        assert_eq!(cpu.read_disp16_ern_l(0, 0x0f10).await.unwrap(), 0x0f0fff0f);

        cpu.er[1] = 0xfffe0a;
        cpu.write_abs24_l(0xfffe00, 0x0f0fff0f).await.unwrap();
        assert_eq!(cpu.read_disp16_ern_l(1, 0xfff6).await.unwrap(), 0x0f0fff0f);
    }

    #[tokio::test]
    async fn test_write_disp24_ern_b() {
        let mut cpu = Cpu::new();
        cpu.er[0] = 0xfff000;
        cpu.write_disp24_ern_b(0, 0x000f10, 0xff).await.unwrap();
        assert_eq!(cpu.read_abs24_b(0xffff10).await.unwrap(), 0xff);

        cpu.er[1] = 0xfffe0a;
        cpu.write_disp24_ern_b(1, 0xfffff6, 0xff).await.unwrap();
        assert_eq!(cpu.read_abs24_b(0xfffe00).await.unwrap(), 0xff);
    }

    #[tokio::test]
    async fn test_read_disp24_ern_b() {
        let mut cpu = Cpu::new();
        cpu.er[0] = 0xfff000;
        cpu.write_abs24_b(0xffff10, 0xff).await.unwrap();
        assert_eq!(cpu.read_disp24_ern_b(0, 0x000f10).await.unwrap(), 0xff);

        cpu.er[1] = 0xfffe0a;
        cpu.write_abs24_b(0xfffe00, 0xff).await.unwrap();
        assert_eq!(cpu.read_disp24_ern_b(1, 0xfffff6).await.unwrap(), 0xff);
    }

    #[tokio::test]
    async fn test_write_disp24_ern_w() {
        let mut cpu = Cpu::new();
        cpu.er[0] = 0xfff000;
        cpu.write_disp24_ern_w(0, 0x000f10, 0x0fff).await.unwrap();
        assert_eq!(cpu.read_abs24_w(0xffff10).await.unwrap(), 0x0fff);

        cpu.er[1] = 0xfffe0a;
        cpu.write_disp24_ern_w(1, 0xfffff6, 0x0fff).await.unwrap();
        assert_eq!(cpu.read_abs24_w(0xfffe00).await.unwrap(), 0x0fff);
    }

    #[tokio::test]
    async fn test_read_disp24_ern_w() {
        let mut cpu = Cpu::new();
        cpu.er[0] = 0xfff000;
        cpu.write_abs24_w(0xffff10, 0x0fff).await.unwrap();
        assert_eq!(cpu.read_disp24_ern_w(0, 0x000f10).await.unwrap(), 0x0fff);

        cpu.er[1] = 0xfffe0a;
        cpu.write_abs24_w(0xfffe00, 0x0fff).await.unwrap();
        assert_eq!(cpu.read_disp24_ern_w(1, 0xfffff6).await.unwrap(), 0x0fff);
    }

    #[tokio::test]
    async fn test_write_disp24_ern_l() {
        let mut cpu = Cpu::new();
        cpu.er[0] = 0xfff000;
        cpu.write_disp24_ern_l(0, 0x000f10, 0x0f0fff0f)
            .await
            .unwrap();
        assert_eq!(cpu.read_abs24_l(0xffff10).await.unwrap(), 0x0f0fff0f);

        cpu.er[1] = 0xfffe0a;
        cpu.write_disp24_ern_l(1, 0xfffff6, 0x0f0fff0f)
            .await
            .unwrap();
        assert_eq!(cpu.read_abs24_l(0xfffe00).await.unwrap(), 0x0f0fff0f);
    }

    #[tokio::test]
    async fn test_read_disp24_ern_l() {
        let mut cpu = Cpu::new();
        cpu.er[0] = 0xfff000;
        cpu.write_abs24_l(0xffff10, 0x0f0fff0f).await.unwrap();
        assert_eq!(
            cpu.read_disp24_ern_l(0, 0x000f10).await.unwrap(),
            0x0f0fff0f
        );

        cpu.er[1] = 0xfffe0a;
        cpu.write_abs24_l(0xfffe00, 0x0f0fff0f).await.unwrap();
        assert_eq!(
            cpu.read_disp24_ern_l(1, 0xfffff6).await.unwrap(),
            0x0f0fff0f
        );
    }
}
