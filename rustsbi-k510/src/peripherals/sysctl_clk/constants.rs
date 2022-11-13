
pub mod base {
    pub const SYSCTL_BASE_ADDR: usize = 0x97000000;
    pub const SYSCTL_CLK_BASE_ADDR: usize = SYSCTL_BASE_ADDR + 0x1000;
}

pub mod osc_clock_freq {
    pub const FREQ_32K: u32 = 32768;
    pub const FREQ_25M: u32 = 25000000;
}

pub mod offsets {
    pub const AX25M_CLK_CFG: usize = 0x00;
    pub const AX25M_MTIMER_CLK_CFG: usize = 0x04;
    pub const RESERVED0: usize = 0x08; // 0x08 0x0c
    pub const AX25P_CLK_CFG: usize = 0x10;
    pub const AX25P_MTIMER_CLK_CFG: usize = 0x14;
    pub const RESERVED1: usize = 0x18; // 0x18 0x1c
    pub const AI_GNNE_ACLK_CFG: usize = 0x20;
    pub const AI_SYSCLK_CFG: usize = 0x24;
    pub const GNNE_SYSCLK_CFG: usize = 0x28;
    pub const RESERVED2: usize = 0x2c; // 0x2c 0x30 0x34 0x38
    pub const I2C2AXI_CFG: usize = 0x3c;
    pub const NOC_CLK_CFG: usize = 0x40;
    pub const RESERVED3: usize = 0x44; // 0x44 0x48 0x4c
    pub const PERI_SYS_BUS_CLK_CFG: usize = 0x50;
    pub const PERI_SYS_BUS_CLK_EN: usize = 0x54;
    pub const RESERVED4: usize = 0x58;
    pub const UART0_SCLK_CFG: usize = 0x5c;
    pub const UART1_SCLK_CFG: usize = 0x60;
    pub const UART2_SCLK_CFG: usize = 0x64;
    pub const UART3_SCLK_CFG: usize = 0x68;
    pub const RESERVED5: usize = 0x6c;
    pub const I2S2_SCLK_CFG: usize = 0x70;
    pub const SPI0_SCLK_CFG: usize = 0x74;
    pub const SPI1_SCLK_CFG: usize = 0x78;
    pub const SPI2_SCLK_CFG: usize = 0x7c;
    pub const SPI3_SCLK_CFG: usize = 0x80;
    pub const AUDIF_SCLK_CFG0: usize = 0x84;
    pub const AUDIF_SCLK_CFG1: usize = 0x88;
    pub const AUDIF_DEVCLK_CFG: usize = 0x8c;
    pub const SEC_SYS_BUS_CLK: usize = 0x90;
    pub const SEC_SYS_BUS_CLK_EN: usize = 0x94;
    pub const OTP_CLK_EN: usize = 0x98;
    pub const RESERVED6: usize = 0x9c;
    pub const SRAM_BUS_CLK_EN: usize = 0xa0;
    pub const RESERVED7: usize = 0xa4; // 0xa4 0xa8 0xac
    pub const SOC_CTL_PCLK_EN: usize = 0xb0;
    pub const SOC_I2C_PCLK_EN: usize = 0xb4;
    pub const I2C0_ICCLK_CFG: usize = 0xb8;
    pub const I2C1_ICCLK_CFG: usize = 0xbc;
    pub const I2C2_ICCLK_CFG: usize = 0xc0;
    pub const I2C3_ICCLK_CFG: usize = 0xc4;
    pub const I2C4_ICCLK_CFG: usize = 0xc8;
    pub const I2C5_ICCLK_CFG: usize = 0xcc;
    pub const I2C6_ICCLK_CFG: usize = 0xd0;
    pub const WDT0_TCLK_CFG: usize = 0xd4;
    pub const WDT1_TCLK_CFG: usize = 0xd8;
    pub const WDT2_TCLK_CFG: usize = 0xdc;
    pub const TIMER_TCLK_SRC: usize = 0xe0;
    pub const TIMER_TCLK_CFG0: usize = 0xe4;
    pub const TIMER_TCLK_CFG1: usize = 0xe8;
    pub const VAD_SCLK_CFG: usize = 0xec;
    pub const RESERVED8: usize = 0xf0; // 0xf0 0xf4 0xf8 0xfc
    pub const STOR_SYS_BUS_CLK_EN: usize = 0x100;
    pub const EMAC_TRX_CLK_CFG: usize = 0x104;
    pub const SD_CARD_CLK_CFG: usize = 0x108;
    pub const SENSOR_CLK_CFG: usize = 0x10c;
    pub const ISP_SYS_PCLK_EN: usize = 0x110;
    pub const ISP_SYS_ACLK_EN: usize = 0x114;
    pub const DISP_SYS_PCLK_EN: usize = 0x118;
    pub const DISP_SYS_ACLK_EN: usize = 0x11c;
    pub const TPG_PIXEL_CLK_CFG: usize = 0x120;
    pub const CSI0_PIXEL_CLK_CFG: usize = 0x124;
    pub const CSI1_PIXEL_CLK_CFG: usize = 0x128;
    pub const CSI2_PIXEL_CLK_CFG: usize = 0x12c;
    pub const CSI3_PIXEL_CLK_CFG: usize = 0x130;
    pub const DISP_PIXEL_CLK_CFG: usize = 0x134;
    pub const MFBC_CLK_CFG: usize = 0x138;
    pub const CSI0_SYS_CLK_CFG: usize = 0x13c;
    pub const CSI1_SYS_CLK_CFG: usize = 0x140;
    pub const CSI2_SYS_CLK_CFG: usize = 0x144;
    pub const CSI3_SYS_CLK_CFG: usize = 0x148;
    pub const DSI_SYS_CLK_CFG: usize = 0x14c;
    pub const H264_ACLK_EN: usize = 0x150;
    pub const USB_CLK_EN: usize = 0x154;
    pub const MIPI_TXDPHY_CLK_EN: usize = 0x158;
    pub const MIPI_RXDPHY_CLK_EN: usize = 0x15c;
    pub const MEM_CTL_CMD_FIFO: usize = 0x160;
    pub const MEM_CTL_CMD_FIFO_STATE: usize = 0x164;
    pub const MEM_CTL_CLK_CFG: usize = 0x168;
    pub const MEM_CTL_DFS_CFG: usize = 0x16c;
}