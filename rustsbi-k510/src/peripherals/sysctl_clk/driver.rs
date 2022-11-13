use core::intrinsics::vtable_align;
use crate::peripherals::sysctl_clk::{
    SysctlBootModeE,
    SysctlClkMulDivMethordE,
    SysctlClkNodeE,
    base,
    osc_clock_freq,
};
use spin::{once::Once};
use k510_pac::{sysctl, SYSCTL};

pub(crate) static SYSCTL: Once<Sysctl> = Once::new();

pub(crate) struct Sysctl(&'static sysctl::RegisterBlock);

impl Sysctl {
    pub unsafe fn new(base: usize) -> Self {
        Self(unsafe{ &*(base as *mut sysctl::RegisterBlock) })
    }
}

pub fn init() {
    SYSCTL.call_once(unsafe{ Sysctl::new(base::SYSCTL_BASE_ADDR) });
}

/* 25M, 32K and PLL0-3 API ,这6个时钟是根时钟 */
/* 获取锁相环 bypass状态, 如果是bypas，则锁相环输出的是25M OSC时钟 */
pub fn sysctl_boot_get_root_clk_bypass(clk: SysctlClkNodeE) -> bool {
    if let Some(sysctl) = SYSCTL.get() {
        match clk {
            SysctlClkNodeE::SysctlClkRootPll0
            | SysctlClkNodeE::SysctlClkRootPll0Div2
            | SysctlClkNodeE::SysctlClkRootPll0Div3
            | SysctlClkNodeE::SysctlClkRootPll0Div4  => {
                if (sysctl.0.pll0_cfg1.read().bits() >> 19) & 0x1 {true} else {false}
            }
            SysctlClkNodeE::SysctlClkRootPll1
            | SysctlClkNodeE::SysctlClkRootPll1Div2
            | SysctlClkNodeE::SysctlClkRootPll1Div3
            | SysctlClkNodeE::SysctlClkRootPll1Div4  => {
                if (sysctl.0.pll1_cfg1.read().bits() >> 19) & 0x1 {true} else {false}
            }
            SysctlClkNodeE::SysctlClkRootPll2
            | SysctlClkNodeE::SysctlClkRootPll2Div2
            | SysctlClkNodeE::SysctlClkRootPll2Div3
            | SysctlClkNodeE::SysctlClkRootPll2Div4  => {
                if (sysctl.0.pll2_cfg1.read().bits() >> 19) & 0x1 {true} else {false}
            }
            SysctlClkNodeE::SysctlClkRootPll3
            | SysctlClkNodeE::SysctlClkRootPll3Div2
            | SysctlClkNodeE::SysctlClkRootPll3Div3
            | SysctlClkNodeE::SysctlClkRootPll3Div4  => {
                if (sysctl.0.pll2_cfg1.read().bits() >> 19) & 0x1 {true} else {false}
            }
            _ => {
                panic!("unsupported root clock!")
            }
        }
    }
    else {
        panic!("SYSCTL is Null!")
    }
}

pub fn sysctl_boot_set_root_clk_bypass(clk: SysctlClkNodeE, enable: bool) {
    let status_bits: u32 = if enable {
        (1 << 19) | (1 << 28)
    }
    else {
        (0 << 19) | (1 << 28)
    };
    if let Some(sysctl) = SYSCTL.get() {
        match clk {
            SysctlClkNodeE::SysctlClkRootPll0
            | SysctlClkNodeE::SysctlClkRootPll0Div2
            | SysctlClkNodeE::SysctlClkRootPll0Div3
            | SysctlClkNodeE::SysctlClkRootPll0Div4  => {
                sysctl.0.pll0_cfg1.write(|w| unsafe{ w.bits(status_bits) });
            }
            SysctlClkNodeE::SysctlClkRootPll1
            | SysctlClkNodeE::SysctlClkRootPll1Div2
            | SysctlClkNodeE::SysctlClkRootPll1Div3
            | SysctlClkNodeE::SysctlClkRootPll1Div4  => {
                sysctl.0.pll1_cfg1.write(|w| unsafe{ w.bits(status_bits) });
            }
            SysctlClkNodeE::SysctlClkRootPll2
            | SysctlClkNodeE::SysctlClkRootPll2Div2
            | SysctlClkNodeE::SysctlClkRootPll2Div3
            | SysctlClkNodeE::SysctlClkRootPll2Div4  => {
                sysctl.0.pll2_cfg1.write(|w| unsafe{ w.bits(status_bits) });
            }
            SysctlClkNodeE::SysctlClkRootPll3
            | SysctlClkNodeE::SysctlClkRootPll3Div2
            | SysctlClkNodeE::SysctlClkRootPll3Div3
            | SysctlClkNodeE::SysctlClkRootPll3Div4  => {
                sysctl.0.pll3_cfg1.write(|w| unsafe{ w.bits(status_bits) });
            }
            _ => {
                panic!("unsupported root clock!")
            }
        }
    }
}

/* Enable pll, enable 25M clock&pll */
/* 打开或者关闭 root时钟 针对锁相环和25M时钟 */
pub fn sysctl_boot_get_root_clk_en(clk: SysctlClkNodeE) -> bool {
    if let Some(sysctl) = SYSCTL.get() {
        let osc_25m_off_status = (( sysctl.0.osc_25m_off.read().bits() >> 0) & 1) == 0;
        match clk {
            SysctlClkNodeE::SysctlClkRootOscIn0 => {
                false
            }
            SysctlClkNodeE::SysctlClkRootOscIn1 => {
                osc_25m_off_status
            }
            SysctlClkNodeE::SysctlClkRootPll0
            | SysctlClkNodeE::SysctlClkRootPll0Div2
            | SysctlClkNodeE::SysctlClkRootPll0Div3
            | SysctlClkNodeE::SysctlClkRootPll0Div4  => {
                osc_25m_off_status && (1 == (sysctl.0.pll0_ctl.read().bits() >> 8) & 1)
            }
            SysctlClkNodeE::SysctlClkRootPll1
            | SysctlClkNodeE::SysctlClkRootPll1Div2
            | SysctlClkNodeE::SysctlClkRootPll1Div3
            | SysctlClkNodeE::SysctlClkRootPll1Div4  => {
                osc_25m_off_status && (1 == (sysctl.0.pll1_ctl.read().bits() >> 8) & 1)
            }
            SysctlClkNodeE::SysctlClkRootPll2
            | SysctlClkNodeE::SysctlClkRootPll2Div2
            | SysctlClkNodeE::SysctlClkRootPll2Div3
            | SysctlClkNodeE::SysctlClkRootPll2Div4  => {
                osc_25m_off_status && (1 == (sysctl.0.pll2_ctl.read().bits() >> 8) & 1)
            }
            SysctlClkNodeE::SysctlClkRootPll3
            | SysctlClkNodeE::SysctlClkRootPll3Div2
            | SysctlClkNodeE::SysctlClkRootPll3Div3
            | SysctlClkNodeE::SysctlClkRootPll3Div4  => {
                osc_25m_off_status && (1 == (sysctl.0.pll3_ctl.read().bits() >> 8) & 1)
            }
            _ => {
                panic!("unsupported leaf node in sysctl_boot_get_root_clk_en!")
            }
        }
    }
    else {
        panic!("SYSCTL is Null!")
    }
}
pub fn sysctl_boot_set_root_clk_en(clk: SysctlClkNodeE, enable: bool){
    let osc_25m_off_status = if enable {
        ((0 << 0) | (1 << 16))
    }
    else {
        ((1 << 0) | (1 << 16))
    };
    let pll_ctl_enable: u32 = (1 << 8) | (1 << 26);
    if let Some(sysctl) = SYSCTL.get() {
        match clk {
            SysctlClkNodeE::SysctlClkRootOscIn0 => {
                sysctl.0.osc_25m_off.write(|w| unsafe{ w.bits(osc_25m_off_status) });
            }
            SysctlClkNodeE::SysctlClkRootPll0
            | SysctlClkNodeE::SysctlClkRootPll0Div2
            | SysctlClkNodeE::SysctlClkRootPll0Div3
            | SysctlClkNodeE::SysctlClkRootPll0Div4  => {
                if enable {
                    sysctl.0.osc_25m_off.write(|w| unsafe{ w.bits(osc_25m_off_status) });
                    sysctl.0.pll0_ctl.write(|w| unsafe{ w.bits(pll_ctl_enable)} );
                }
            }
            SysctlClkNodeE::SysctlClkRootPll1
            | SysctlClkNodeE::SysctlClkRootPll1Div2
            | SysctlClkNodeE::SysctlClkRootPll1Div3
            | SysctlClkNodeE::SysctlClkRootPll1Div4  => {
                if enable {
                    sysctl.0.osc_25m_off.write(|w| unsafe{ w.bits(osc_25m_off_status) });
                    sysctl.0.pll1_ctl.write(|w| unsafe{ w.bits(pll_ctl_enable)} );
                }
            }
            SysctlClkNodeE::SysctlClkRootPll2
            | SysctlClkNodeE::SysctlClkRootPll2Div2
            | SysctlClkNodeE::SysctlClkRootPll2Div3
            | SysctlClkNodeE::SysctlClkRootPll2Div4  => {
                if enable {
                    sysctl.0.osc_25m_off.write(|w| unsafe{ w.bits(osc_25m_off_status) });
                    sysctl.0.pll2_ctl.write(|w| unsafe{ w.bits(pll_ctl_enable)} );
                }
            }
            SysctlClkNodeE::SysctlClkRootPll3
            | SysctlClkNodeE::SysctlClkRootPll3Div2
            | SysctlClkNodeE::SysctlClkRootPll3Div3
            | SysctlClkNodeE::SysctlClkRootPll3Div4  => {
                if enable {
                    sysctl.0.osc_25m_off.write(|w| unsafe{ w.bits(osc_25m_off_status) });
                    sysctl.0.pll3_ctl.write(|w| unsafe{ w.bits(pll_ctl_enable)} );
                }
            }
            _ => {
                panic!("unsupported leaf node in sysctl_boot_get_root_clk_en!")
            }
        }
    }
    else {
        panic!("SYSCTL is Null!")
    }
}

/* 获取锁相环锁定状态 */
pub fn sysctl_boot_get_root_clk_lock(clk: SysctlClkNodeE) -> bool {
    if let Some(sysctl) = SYSCTL.get() {
        match clk {
            SysctlClkNodeE::SysctlClkRootPll0
            | SysctlClkNodeE::SysctlClkRootPll0Div2
            | SysctlClkNodeE::SysctlClkRootPll0Div3
            | SysctlClkNodeE::SysctlClkRootPll0Div4  => {
                (sysctl.0.pll0_stat.read().bits() >> 0) & 1 != 0
            }
            SysctlClkNodeE::SysctlClkRootPll1
            | SysctlClkNodeE::SysctlClkRootPll1Div2
            | SysctlClkNodeE::SysctlClkRootPll1Div3
            | SysctlClkNodeE::SysctlClkRootPll1Div4  => {
                 (sysctl.0.pll1_stat.read().bits() >> 0) & 1 != 0
            }
            SysctlClkNodeE::SysctlClkRootPll2
            | SysctlClkNodeE::SysctlClkRootPll2Div2
            | SysctlClkNodeE::SysctlClkRootPll2Div3
            | SysctlClkNodeE::SysctlClkRootPll2Div4  => {
                (sysctl.0.pll2_stat.read().bits() >> 0) & 1 != 0
            }
            SysctlClkNodeE::SysctlClkRootPll3
            | SysctlClkNodeE::SysctlClkRootPll3Div2
            | SysctlClkNodeE::SysctlClkRootPll3Div3
            | SysctlClkNodeE::SysctlClkRootPll3Div4  => {
                (sysctl.0.pll3_stat.read().bits() >> 0) & 1 != 0
            }
            _ => {
                panic!("unsupported leaf node in sysctl_boot_get_root_clk_lock!")
            }
        }
    }
    else {
        panic!("SYSCTL is Null!")
    }
}

fn sysctl_boot_set_root_clk_pwroff(clk: SysctlClkNodeE) {
    let ctl: u32 = (1 << 0)|(1 << 24);
    if let Some(sysctl) = SYSCTL.get() {
        match clk {
            SysctlClkNodeE::SysctlClkRootPll0
            | SysctlClkNodeE::SysctlClkRootPll0Div2
            | SysctlClkNodeE::SysctlClkRootPll0Div3
            | SysctlClkNodeE::SysctlClkRootPll0Div4  => {
                sysctl.0.pll0_ctl.write(|w| unsafe{ w.bits(ctl) });
            }
            SysctlClkNodeE::SysctlClkRootPll1
            | SysctlClkNodeE::SysctlClkRootPll1Div2
            | SysctlClkNodeE::SysctlClkRootPll1Div3
            | SysctlClkNodeE::SysctlClkRootPll1Div4  => {
                sysctl.0.pll1_ctl.write(|w| unsafe{ w.bits(ctl) });
            }
            SysctlClkNodeE::SysctlClkRootPll2
            | SysctlClkNodeE::SysctlClkRootPll2Div2
            | SysctlClkNodeE::SysctlClkRootPll2Div3
            | SysctlClkNodeE::SysctlClkRootPll2Div4  => {
                sysctl.0.pll2_ctl.write(|w| unsafe{ w.bits(ctl) });
            }
            SysctlClkNodeE::SysctlClkRootPll3
            | SysctlClkNodeE::SysctlClkRootPll3Div2
            | SysctlClkNodeE::SysctlClkRootPll3Div3
            | SysctlClkNodeE::SysctlClkRootPll3Div4  => {
                sysctl.0.pll3_ctl.write(|w| unsafe{ w.bits(ctl) });
            }
            _ => {
                panic!("unsupported leaf node in sysctl_boot_set_root_clk_pwroff!")
            }
        }
    }
    else {
        panic!("SYSCTL is Null!")
    }
}

/* 获取root时钟频率 */
pub fn sysctl_boot_get_root_clk_freq(clk: SysctlClkNodeE) -> u32 {
    let mut refdiv: u32 = 0;
    let mut outdiv: u32 = 0;
    let mut fbdiv: u32 = 0;
    let mut freq: u32 = 0;
    if let Some(sysctl) = SYSCTL.get() {
        match clk {
            SysctlClkNodeE::SysctlClkRootOscIn0 => {
                osc_clock_freq::FREQ_25M  /* 25MHz */
            }
            SysctlClkNodeE::SysctlClkRootOscIn1 => {
                osc_clock_freq::FREQ_32K  /* 32768 */
            }
            SysctlClkNodeE::SysctlClkRootPll0
            | SysctlClkNodeE::SysctlClkRootPll0Div2
            | SysctlClkNodeE::SysctlClkRootPll0Div3
            | SysctlClkNodeE::SysctlClkRootPll0Div4  => {
                if sysctl_boot_get_root_clk_bypass(clk) {
                    freq = osc_clock_freq::FREQ_25M;
                }
                else {
                    let cfg = sysctl.0.pll0_cfg0.read().bits();
                    refdiv = (cfg >> 16) & 0x3F;    /* bit 16~21 */
                    outdiv = (cfg >> 24) & 0xF;     /* bit 24~27 */
                    fbdiv  = (cfg >> 0)  & 0xFFF;   /* bit 0~11 */
                    freq = ((osc_clock_freq::FREQ_25M as f64) * ((fbdiv+1) as f64) / ((refdiv+1) as f64) / ((outdiv+1) as f64)) as u32;
                }
                match clk {
                    SysctlClkNodeE::SysctlClkRootPll0 => {
                        freq
                    }
                    SysctlClkNodeE::SysctlClkRootPll0Div2 => {
                        freq / 2
                    }
                    SysctlClkNodeE::SysctlClkRootPll0Div3 => {
                        freq / 3
                    }
                    SysctlClkNodeE::SysctlClkRootPll0Div4 => {
                        freq / 4
                    }
                    _ => panic!("unsupported leaf node in sysctl_boot_get_root_clk_freq!")
                }
            }
            SysctlClkNodeE::SysctlClkRootPll1
            | SysctlClkNodeE::SysctlClkRootPll1Div2
            | SysctlClkNodeE::SysctlClkRootPll1Div3
            | SysctlClkNodeE::SysctlClkRootPll1Div4  => {
                if sysctl_boot_get_root_clk_bypass(clk) {
                    freq = osc_clock_freq::FREQ_25M;
                }
                else {
                    let cfg = sysctl.0.pll1_cfg0.read().bits();
                    refdiv = (cfg >> 16) & 0x3F;    /* bit 16~21 */
                    outdiv = (cfg >> 24) & 0xF;     /* bit 24~27 */
                    fbdiv  = (cfg >> 0)  & 0xFFF;   /* bit 0~11 */
                    freq = ((osc_clock_freq::FREQ_25M as f64) * ((fbdiv+1) as f64) / ((refdiv+1) as f64) / ((outdiv+1) as f64)) as u32;
                }
                match clk {
                    SysctlClkNodeE::SysctlClkRootPll1 => {
                        freq
                    }
                    SysctlClkNodeE::SysctlClkRootPll1Div2 => {
                        freq / 2
                    }
                    SysctlClkNodeE::SysctlClkRootPll1Div3 => {
                        freq / 3
                    }
                    SysctlClkNodeE::SysctlClkRootPll1Div4 => {
                        freq / 4
                    }
                    _ => panic!("unsupported leaf node in sysctl_boot_get_root_clk_freq!")
                }
            }
            SysctlClkNodeE::SysctlClkRootPll2
            | SysctlClkNodeE::SysctlClkRootPll2Div2
            | SysctlClkNodeE::SysctlClkRootPll2Div3
            | SysctlClkNodeE::SysctlClkRootPll2Div4  => {
                if sysctl_boot_get_root_clk_bypass(clk) {
                    freq = osc_clock_freq::FREQ_25M;
                }
                else {
                    let cfg = sysctl.0.pll2_cfg0.read().bits();
                    refdiv = (cfg >> 16) & 0x3F;    /* bit 16~21 */
                    outdiv = (cfg >> 24) & 0xF;     /* bit 24~27 */
                    fbdiv  = (cfg >> 0)  & 0xFFF;   /* bit 0~11 */
                    freq = ((osc_clock_freq::FREQ_25M as f64) * ((fbdiv+1) as f64) / ((refdiv+1) as f64) / ((outdiv+1) as f64)) as u32;
                }
                match clk {
                    SysctlClkNodeE::SysctlClkRootPll2 => {
                        freq
                    }
                    SysctlClkNodeE::SysctlClkRootPll2Div2 => {
                        freq / 2
                    }
                    SysctlClkNodeE::SysctlClkRootPll2Div3 => {
                        freq / 3
                    }
                    SysctlClkNodeE::SysctlClkRootPll2Div4 => {
                        freq / 4
                    }
                    _ => panic!("unsupported leaf node in sysctl_boot_get_root_clk_freq!")
                }
            }
            SysctlClkNodeE::SysctlClkRootPll3
            | SysctlClkNodeE::SysctlClkRootPll3Div2
            | SysctlClkNodeE::SysctlClkRootPll3Div3
            | SysctlClkNodeE::SysctlClkRootPll3Div4  => {
                if sysctl_boot_get_root_clk_bypass(clk) {
                    freq = osc_clock_freq::FREQ_25M;
                }
                else {
                    let cfg = sysctl.0.pll3_cfg0.read().bits();
                    refdiv = (cfg >> 16) & 0x3F;    /* bit 16~21 */
                    outdiv = (cfg >> 24) & 0xF;     /* bit 24~27 */
                    fbdiv  = (cfg >> 0)  & 0xFFF;   /* bit 0~11 */
                    freq = ((osc_clock_freq::FREQ_25M as f64) * ((fbdiv+1) as f64) / ((refdiv+1) as f64) / ((outdiv+1) as f64)) as u32;
                }
                match clk {
                    SysctlClkNodeE::SysctlClkRootPll3 => {
                        freq
                    }
                    SysctlClkNodeE::SysctlClkRootPll3Div2 => {
                        freq / 2
                    }
                    SysctlClkNodeE::SysctlClkRootPll3Div3 => {
                        freq / 3
                    }
                    SysctlClkNodeE::SysctlClkRootPll3Div4 => {
                        freq / 4
                    }
                    _ => panic!("unsupported leaf node in sysctl_boot_get_root_clk_freq!")
                }
            }
            _ => panic!("unsupported leaf node in sysctl_boot_get_root_clk_freq!")
        }
    }
    else {
        panic!("SYSCTL is Null!")
    }
}

/* 设置锁相环时钟频率 计算公式 pll_out_freq = (double)OSC_CLOCK_FREQ_25M * (double)(fbdiv+1) / (double)(refdiv+1) / (double)(outdiv+1) */
/*freq = (double)OSC_CLOCK_FREQ_25M * (double)(fbdiv+1) / (double)(refdiv+1) / (double)(outdiv+1)
  please configure the PLL frequency according to the above frequency division coefficient
  Note: when configuring, you can't configure yourself. For example, the PLL attached to CPU can't stop
  before configuring the PLL of CPU, switch the clock first and then configure it. After configuration, switch it back
*/
pub fn sysctl_boot_set_root_clk_freq(
    clk: SysctlClkNodeE,
    fbdiv: u32,
    refdiv: u32,
    outdiv: u32
) -> bool {
    let mut wait_us: u32 = 100;
    match clk {
        SysctlClkNodeE::SysctlClkRootPll0
        | SysctlClkNodeE::SysctlClkRootPll1
        | SysctlClkNodeE::SysctlClkRootPll2
        | SysctlClkNodeE::SysctlClkRootPll3 => {}
        _ => panic!("unsupported leaf node in sysctl_boot_set_root_clk_freq!")
    }

    /*1. poweroff pll*/
    sysctl_boot_set_root_clk_pwroff(clk.clone());

    let cfg: u32 = ((fbdiv &  0xFFF) << 0) | ((refdiv &  0x3F) << 16) | ((outdiv & 0xF) << 24) | (1 << 28) | (1 << 29) | (1 << 30);
    let ctl: u32 = (1 << 4)|(1 << 25);
    if let Some(sysctl) = SYSCTL.get() {
        match clk {
            SysctlClkNodeE::SysctlClkRootPll0 => {
                /* 2. config divide*/
                sysctl.0.pll0_cfg0.write(|w| unsafe{ w.bits(cfg) });
                /* 3. init pll. init will pwrup pll */
                sysctl.0.pll0_ctl.write(|w| unsafe{ w.bits(ctl) });
            }
            SysctlClkNodeE::SysctlClkRootPll1 => {
                sysctl.0.pll1_cfg0.write(|w| unsafe{ w.bits(cfg) });
                sysctl.0.pll1_ctl.write(|w| unsafe{ w.bits(ctl) });
            }
            SysctlClkNodeE::SysctlClkRootPll2 => {
                sysctl.0.pll2_cfg0.write(|w| unsafe{ w.bits(cfg) });
                sysctl.0.pll2_ctl.write(|w| unsafe{ w.bits(ctl) });
            }
            SysctlClkNodeE::SysctlClkRootPll3 => {
                sysctl.0.pll3_cfg0.write(|w| unsafe{ w.bits(cfg) });
                sysctl.0.pll3_ctl.write(|w| unsafe{ w.bits(ctl) });
            }
            _ => panic!("unsupported leaf node in sysctl_boot_set_root_clk_freq!")
        }
    }
    else {
        panic!("SYSCTL is Null!")
    }

    /* 4. check lock status */
    loop {
        if false == sysctl_boot_get_root_clk_lock(clk.clone())
        {
            wait_us -= 1;
            if wait_us == 0 {
                false;
            }
        }
        else{
            true;
        }
    }
}

/* 下面的API是针对时钟树的trunk和leaf节点, 换言之,就是支持除了OSC25M, OSC32K和PLL0-PPL3之外的其他时钟 */
/* 设置时钟数上叶子节点时钟源, 请根据时钟树来设置, 很多时钟节点只有一个时钟源，因此设置会返回false */
pub fn sysctl_clk_set_leaf_parent(leaf: SysctlClkNodeE, parent: SysctlClkNodeE) -> bool {
    if let Some(sysctl) = SYSCTL.get() {
        match leaf {
            /*---------------------------AX25MP------------------------------------*/
            SysctlClkNodeE::SysctlClkAx25mSrc => {
                let mut cfg: u32 = 0;
                match parent {
                    SysctlClkNodeE::SysctlClkRootPll2Div3 => {
                        cfg = ((0 << 0) | (1 << 24));
                    }
                    SysctlClkNodeE::SysctlClkRootPll0 => {
                        cfg = ((1 << 0) | (1 << 24));
                    }
                    SysctlClkNodeE::SysctlClkRootPll2Div2 => {
                        cfg = ((2 << 0) | (1 << 24))
                    }
                    _ => panic!("unsupported leaf node in sysctl_clk_set_leaf_parent!")
                }
                sysctl.0.ax25m_clk_cfg.write(|w| unsafe{ w.bits(cfg) });
                true
            }
            SysctlClkNodeE::SysctlClkAx25mCore0
            | SysctlClkNodeE::SysctlClkAx25mCore1
            | SysctlClkNodeE::SysctlClkAx25mCore0Dc
            | SysctlClkNodeE::SysctlClkAx25mCore1Dc
            | SysctlClkNodeE::SysctlClkAx25mMctl => {
                false
            }
            SysctlClkNodeE::SysctlClkAx25mMtimer => {
                let mut cfg: u32 = 0;
                match parent {
                    SysctlClkNodeE::SysctlClkRootOscIn1 => {
                        cfg = ((0 << 0) | (1 << 24))
                    }
                    SysctlClkNodeE::SysctlClkRootOscIn0 => {
                        cfg = ((1 << 0) | (1 << 24))
                    }
                    _ => panic!("unsupported leaf node in sysctl_clk_set_leaf_parent!")
                }
                sysctl.0.ax25m_mtimer_clk_cfg.write(|w| unsafe{ w.bits(cfg) })
                true
            }
            /*---------------------------AX25P------------------------------------*/
            // TODO - 完成剩余内容


            _ => panic!("unsupported leaf node in sysctl_clk_set_leaf_parent!")
        }
    }
    else {
        panic!("SYSCTL is Null!")
    }
}

/* 获取时钟树上叶子节点时钟源 */
pub fn sysctl_clk_get_leaf_parent(leaf: SysctlClkNodeE) -> SysctlClkNodeE {
    // TODO
    SysctlClkNodeE::SysctlClkWdt2Tclk
}

/* 设置时钟节点enable,注意:只设置本时钟节点的enable，不会设置上游时钟的enable。
   同linux kernel的区别: linux kernel clock framework 会自动设置上游时钟的enable，测试代码没有kernel框架，因此只设置本节点时钟的enable */

pub fn sysctl_clk_set_leaf_en(leaf: SysctlClkNodeE, enable: bool) {
    // TODO
}

/* 获取本时钟节点的enable状态 */
pub fn sysctl_clk_get_leaf_en(leaf: SysctlClkNodeE) -> bool {
    // TODO
    false
}

/* 获取本时钟节点的分频系数 */
pub fn sysctl_clk_set_leaf_div(leaf: SysctlClkNodeE, numerator: u32, denominator: u32) -> bool {
    // TODO
    false
}

/* 获取本时钟节点的分频系数 */
pub fn sysctl_clk_get_leaf_div(leaf: SysctlClkNodeE) -> f64 {
    // TODO
    0.0
}

/* 设置本时钟节点的相位 */
pub fn sysctl_clk_set_phase(leaf: SysctlClkNodeE, degree: u32) -> bool {
    // TODO
    false
}

/* 获取本时钟节点的相位 */
pub fn sysctl_clk_get_phase(leaf: SysctlClkNodeE) -> u32 {
    // TODO
    0
}

/* calc clock freqency */
/* 计算当前时钟节点的频率, 这个API会搜索整个时钟路径，从时钟源开始计算每一级的分频，最终得出当前时钟频率 */
pub fn sysctl_clk_get_leaf_freq(leaf: SysctlClkNodeE) -> u32 {
    // TODO
    0
}


/* 辅助计算函数，本函数会根据父节点时钟/实际需要输出的时钟/分频配置方法 计算出最合适的分频系数 */
pub fn sysctl_clk_find_approximate(
    mul_min: u32,
    mul_max: u32,
    div_min: u32,
    div_max: u32,
    method: SysctlClkMulDivMethordE,
    rate: u64,
    parent_rate: u64,
    div: *mut u32,
    mul: *mut u32
) -> i32{
    // TODO
    -1
}

pub fn sysctl_boot_get_boot_mode() -> SysctlBootModeE {
    // TODO
    SysctlBootModeE::SysctlBootMax
}