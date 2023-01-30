use crate::peripherals::sysctl_clk::{
    SysctlBootModeE,
    SysctlClkMulDivMethordE,
    SysctlClkNodeE,
    base,
    osc_clock_freq,
};
use spin::once::Once;
use k510_pac::sysctl;

pub(crate) static SYSCTL: Once<Sysctl> = Once::new();

pub(crate) struct Sysctl(&'static sysctl::RegisterBlock);

impl Sysctl {
    pub unsafe fn new(base: usize) -> Self {
        Self(unsafe{ &*(base as *mut sysctl::RegisterBlock) })
    }
}

unsafe impl Sync for Sysctl {}

unsafe impl Send for Sysctl {}

#[allow(unused)]
pub fn init() {
    SYSCTL.call_once(unsafe{|| Sysctl::new(base::SYSCTL_BASE_ADDR) });
}

/* 25M, 32K and PLL0-3 API ,这6个时钟是根时钟 */
/* 获取锁相环 bypass状态, 如果是bypas，则锁相环输出的是25M OSC时钟 */
#[allow(unused)]
pub fn sysctl_boot_get_root_clk_bypass(clk: SysctlClkNodeE) -> bool {
    if let Some(sysctl) = SYSCTL.get() {
        match clk {
            SysctlClkNodeE::SysctlClkRootPll0
            | SysctlClkNodeE::SysctlClkRootPll0Div2
            | SysctlClkNodeE::SysctlClkRootPll0Div3
            | SysctlClkNodeE::SysctlClkRootPll0Div4  => {
                (sysctl.0.pll0_cfg1.read().bits() >> 19) & 0x1 != 0
            }
            SysctlClkNodeE::SysctlClkRootPll1
            | SysctlClkNodeE::SysctlClkRootPll1Div2
            | SysctlClkNodeE::SysctlClkRootPll1Div3
            | SysctlClkNodeE::SysctlClkRootPll1Div4  => {
                (sysctl.0.pll1_cfg1.read().bits() >> 19) & 0x1 != 0
            }
            SysctlClkNodeE::SysctlClkRootPll2
            | SysctlClkNodeE::SysctlClkRootPll2Div2
            | SysctlClkNodeE::SysctlClkRootPll2Div3
            | SysctlClkNodeE::SysctlClkRootPll2Div4  => {
                (sysctl.0.pll2_cfg1.read().bits() >> 19) & 0x1 != 0
            }
            SysctlClkNodeE::SysctlClkRootPll3
            | SysctlClkNodeE::SysctlClkRootPll3Div2
            | SysctlClkNodeE::SysctlClkRootPll3Div3
            | SysctlClkNodeE::SysctlClkRootPll3Div4  => {
                (sysctl.0.pll3_cfg1.read().bits() >> 19) & 0x1 != 0
            }
            _ => {
                panic!("unsupported root clock!")
                // return false
            }
        }
    }
    else {
        panic!("SYSCTL is Null!")
    }
}

#[allow(unused)]
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
#[allow(unused)]
pub fn sysctl_boot_get_root_clk_en(clk: SysctlClkNodeE) -> bool {
    if let Some(sysctl) = SYSCTL.get() {
        let osc_25m_off_status = (( sysctl.0.osc_25m_off.read().bits() >> 0) & 1) == 0;
        match clk {
            SysctlClkNodeE::SysctlClkRootOscIn0 => {
                osc_25m_off_status
            }
            SysctlClkNodeE::SysctlClkRootOscIn1 => {
                true
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

#[allow(unused)]
pub fn sysctl_boot_set_root_clk_en(clk: SysctlClkNodeE, enable: bool){
    let osc_25m_off_status = if enable {
        (0 << 0) | (1 << 16)
    }
    else {
        (1 << 0) | (1 << 16)
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
#[allow(unused)]
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

#[allow(unused)]
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
#[allow(unused)]
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
                if sysctl_boot_get_root_clk_bypass(clk.clone()) {
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
                if sysctl_boot_get_root_clk_bypass(clk.clone()) {
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
                if sysctl_boot_get_root_clk_bypass(clk.clone()) {
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
                if sysctl_boot_get_root_clk_bypass(clk.clone()) {
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
#[allow(unused)]
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
        match clk.clone() {
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
#[allow(unused)]
pub fn sysctl_clk_set_leaf_parent(leaf: SysctlClkNodeE, parent: SysctlClkNodeE) -> bool {
    if let Some(sysctl) = SYSCTL.get() {
        match leaf {
            /*---------------------------AX25MP------------------------------------*/
            SysctlClkNodeE::SysctlClkAx25mSrc => {
                let mut cfg: u32 = 0;
                match parent {
                    SysctlClkNodeE::SysctlClkRootPll2Div3 => {
                        cfg = (0 << 0) | (1 << 24);
                    }
                    SysctlClkNodeE::SysctlClkRootPll0 => {
                        cfg = (1 << 0) | (1 << 24);
                    }
                    SysctlClkNodeE::SysctlClkRootPll2Div2 => {
                        cfg = (2 << 0) | (1 << 24)
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
                        cfg = (0 << 0) | (1 << 24)
                    }
                    SysctlClkNodeE::SysctlClkRootOscIn0 => {
                        cfg = (1 << 0) | (1 << 24)
                    }
                    _ => panic!("unsupported leaf node in sysctl_clk_set_leaf_parent!")
                }
                sysctl.0.ax25m_mtimer_clk_cfg.write(|w| unsafe{ w.bits(cfg) });
                true
            }
            /*---------------------------AX25P------------------------------------*/
            SysctlClkNodeE::SysctlClkAx25pSrc => {
                let mut cfg: u32 = 0;
                match parent {
                    SysctlClkNodeE::SysctlClkRootPll2Div3 => {
                        cfg = ((0 << 0) | (1 << 24));
                    }
                    SysctlClkNodeE::SysctlClkRootPll0 => {
                        cfg = ((1 << 0) | (1 << 24));
                    }
                    _ => panic!("unsupported leaf node in sysctl_clk_set_leaf_parent!")
                }
                sysctl.0.ax25p_clk_cfg.write(|w| unsafe{ w.bits(cfg) });
                true
            }
            SysctlClkNodeE::SysctlClkAx25pCore 
            | SysctlClkNodeE::SysctlClkAx25pLm  => {
                false
            }
            SysctlClkNodeE::SysctlClkAx25pMtimer => {
                let mut cfg: u32 = 0;
                match parent {
                    SysctlClkNodeE::SysctlClkRootOscIn1 => {
                        cfg = ((0 << 0) | (1 << 24));
                    }
                    SysctlClkNodeE::SysctlClkRootOscIn0 => {
                        cfg = ((1 << 0) | (1 << 24));
                    }
                    _ => panic!("unsupported leaf node in sysctl_clk_set_leaf_parent!")
                }
                sysctl.0.ax25p_mtimer_clk_cfg.write(|w| unsafe{ w.bits(cfg) });
                true
            }

            /*---------------------------GNNE------------------------------------*/
            SysctlClkNodeE::SysctlClkGnneSys => {
                let mut cfg: u32 = 0;
                match parent {
                    SysctlClkNodeE::SysctlClkRootPll0Div2 => {
                        cfg = ((0 << 0) | (1 << 24));
                    }
                    SysctlClkNodeE::SysctlClkRootPll1Div2 => {
                        cfg = ((1 << 0) | (1 << 24));
                    }
                    _ => panic!("unsupported leaf node in sysctl_clk_set_leaf_parent!")
                }
                sysctl.0.gnne_sysclk_cfg.write(|w| unsafe{ w.bits(cfg) });
                true
            }
            SysctlClkNodeE::SysctlClkGnneAxi => {
                let mut cfg: u32 = 0;
                match parent {
                    SysctlClkNodeE::SysctlClkRootPll0Div2 => {
                        cfg = ((0 << 0) | (1 << 24));
                    }
                    SysctlClkNodeE::SysctlClkRootPll1Div2 => {
                        cfg = ((1 << 0) | (1 << 24));
                    }
                    _ => panic!("unsupported leaf node in sysctl_clk_set_leaf_parent!")
                }
                sysctl.0.gnne_aclk_cfg.write(|w| unsafe { w.bits(cfg) });
                true
            }
            SysctlClkNodeE::SysctlClkGnneAxiNoc
            | SysctlClkNodeE::SysctlClkGnneAxiMctl => {
                false
            }

            /*---------------------------NOC0------------------------------------*/
            SysctlClkNodeE::SysctlClkNocClk0 => {
                let mut cfg: u32 = 0;
                match parent {
                    SysctlClkNodeE::SysctlClkRootPll0Div2 => {
                        cfg = ((0 << 0) | (1 << 24));
                    }
                    SysctlClkNodeE::SysctlClkRootPll1Div2 => {
                        cfg = ((1 << 0) | (1 << 24));
                    }
                    _ => panic!("unsupported leaf node in sysctl_clk_set_leaf_parent!")
                }
                sysctl.0.noc_clk_cfg.write(|w| unsafe { w.bits(cfg) });
                true
            }
            SysctlClkNodeE::SysctlClkNocClk0Div2
            | SysctlClkNodeE::SysctlClkNocClk0Div3 
            | SysctlClkNodeE::SysctlClkNocClk0Div4 => {
                false
            }
            SysctlClkNodeE::SysctlClkNocClk0PeriDmaAxi
            | SysctlClkNodeE::SysctlClkNocClk0SysDmaAxi
            | SysctlClkNodeE::SysctlClkNocClk0Sram0Axi
            | SysctlClkNodeE::SysctlClkNocClk0Sram1Axi
            | SysctlClkNodeE::SysctlClkNocClk0AxiP3 => {
                false
            }
            SysctlClkNodeE::SysctlClkNocClk0Div2MctlAhb
            | SysctlClkNodeE::SysctlClkNocClk0Div2UsbAhb
            | SysctlClkNodeE::SysctlClkNocClk0Div2SdSlvAhb => {
                false
            }
            SysctlClkNodeE::SysctlClkNocClk0Div3Sd0Ahb
            | SysctlClkNodeE::SysctlClkNocClk0Div3Sd1Ahb
            | SysctlClkNodeE::SysctlClkNocClk0Div3Sd2Ahb
            | SysctlClkNodeE::SysctlClkNocClk0Div3EmacAhb
            | SysctlClkNodeE::SysctlClkNocClk0Div3PeriDmaApb
            | SysctlClkNodeE::SysctlClkNocClk0Div3SysDmaApb => {
                false
            }
            SysctlClkNodeE::SysctlClkNocClk0Div4Wdt0Apb
            | SysctlClkNodeE::SysctlClkNocClk0Div4Wdt1Apb
            | SysctlClkNodeE::SysctlClkNocClk0Div4Wdt2Apb
            | SysctlClkNodeE::SysctlClkNocClk0Div4TimerApb
            | SysctlClkNodeE::SysctlClkNocClk0Div4RtcApb
            | SysctlClkNodeE::SysctlClkNocClk0Div4GpioApb
            | SysctlClkNodeE::SysctlClkNocClk0Div4IomuxApb
            | SysctlClkNodeE::SysctlClkNocClk0Div4I2c0Apb
            | SysctlClkNodeE::SysctlClkNocClk0Div4I2c1Apb
            | SysctlClkNodeE::SysctlClkNocClk0Div4I2c2Apb
            | SysctlClkNodeE::SysctlClkNocClk0Div4I2c3Apb
            | SysctlClkNodeE::SysctlClkNocClk0Div4I2c4Apb
            | SysctlClkNodeE::SysctlClkNocClk0Div4I2c5Apb
            | SysctlClkNodeE::SysctlClkNocClk0Div4I2c6Apb
            | SysctlClkNodeE::SysctlClkNocClk0Div4PwmApb
            | SysctlClkNodeE::SysctlClkNocClk0Div4MailboxApb
            | SysctlClkNodeE::SysctlClkNocClk0Div4VadApb => {
                false
            }
            
            /*---------------------------NOC1------------------------------------*/
            SysctlClkNodeE::SysctlClkNocClk1 => {
                false
            }

            SysctlClkNodeE::SysctlClkNocClk1AxiMctl
            | SysctlClkNodeE::SysctlClkNocClk1H264Axi
            | SysctlClkNodeE::SysctlClkNocClk1VoAxi
            | SysctlClkNodeE::SysctlClkNocClk1TwodAxi
            | SysctlClkNodeE::SysctlClkNocClk1MfbcAxi
            | SysctlClkNodeE::SysctlClkNocClk1ViAxi
            | SysctlClkNodeE::SysctlClkNocClk1IspF2kAxi
            | SysctlClkNodeE::SysctlClkNocClk1IspR2kAxi
            | SysctlClkNodeE::SysctlClkNocClk1IspTofAxi

            | SysctlClkNodeE::SysctlClkNocClk1PeriApb

            | SysctlClkNodeE::SysctlClkNocClk1PeriApbUart0Apb
            | SysctlClkNodeE::SysctlClkNocClk1PeriApbUart1Apb
            | SysctlClkNodeE::SysctlClkNocClk1PeriApbUart2Apb
            | SysctlClkNodeE::SysctlClkNocClk1PeriApbUart3Apb
            | SysctlClkNodeE::SysctlClkNocClk1PeriApbAudioApb
            
            | SysctlClkNodeE::SysctlClkNocClk1PeriAhb
            | SysctlClkNodeE::SysctlClkNocClk1PeriAhbSpi0Ahb
            | SysctlClkNodeE::SysctlClkNocClk1PeriAhbSpi1Ahb
            | SysctlClkNodeE::SysctlClkNocClk1PeriAhbSpi2Ahb
            | SysctlClkNodeE::SysctlClkNocClk1PeriAhbSpi3Ahb => {
                false
            }

            SysctlClkNodeE::SysctlClkNocClk1Div4 => {
                false
            }

            SysctlClkNodeE::SysctlClkNocClk1Csi0Apb
            | SysctlClkNodeE::SysctlClkNocClk1Csi1Apb
            | SysctlClkNodeE::SysctlClkNocClk1Csi2Apb
            | SysctlClkNodeE::SysctlClkNocClk1Csi3Apb
            | SysctlClkNodeE::SysctlClkNocClk1F2kApb
            | SysctlClkNodeE::SysctlClkNocClk1R2kApb
            | SysctlClkNodeE::SysctlClkNocClk1TofApb
            | SysctlClkNodeE::SysctlClkNocClk1MfbcApb
            | SysctlClkNodeE::SysctlClkNocClk1MipiCornerApb
            | SysctlClkNodeE::SysctlClkNocClk1ViApb => {
                false
            }

            /*---------------------------DISPLAY------------------------------------*/
            SysctlClkNodeE::SysctlClkDispSysAndApbClkDiv => {
                false
            }
            SysctlClkNodeE::SysctlClkDispSysAndApbClkDivDsiApb
            | SysctlClkNodeE::SysctlClkDispSysAndApbClkDivDsiSystem
            | SysctlClkNodeE::SysctlClkDispSysAndApbClkDivVoApb
            | SysctlClkNodeE::SysctlClkDispSysAndApbClkDivTwodApb
            | SysctlClkNodeE::SysctlClkDispSysAndApbClkDivBt1120Apb => {
                false
            }
            
            /*---------------------------csi0_system_clk-----------------------------*/
            SysctlClkNodeE::SysctlClkCsi0System => {
                false
            }

            /*---------------------------csi1_system_clk-----------------------------*/
            SysctlClkNodeE::SysctlClkCsi1System => {
                false
            }

            /*---------------------------csi0_pixel_clk------------------------------*/
            SysctlClkNodeE::SysctlClkCsi0Pixel => {
                false
            }

            /*---------------------------csi1_pixel_clk------------------------------*/
            SysctlClkNodeE::SysctlClkCsi1Pixel => {
                let mut cfg: u32 = 0;
                match parent {
                    SysctlClkNodeE::SysctlClkRootPll2Div4 => {
                        cfg = ((0 << 9) | (1 << 26));
                    }
                    SysctlClkNodeE::SysctlClkRootPll2Div3 => {
                        cfg = ((1 << 9) | (1 << 26));
                    }
                    SysctlClkNodeE::SysctlClkRootPll0 => {
                        cfg = ((2 << 9) | (1 << 26));
                    }
                    SysctlClkNodeE::SysctlClkRootPll1 => {
                        cfg = ((3 << 9) | (1 << 26));
                    }
                    _ => panic!("unsupported leaf node in sysctl_clk_set_leaf_parent!")
                }
                sysctl.0.csi_pixel_clk_cfg[1].write(|w| unsafe{ w.bits(cfg) });
                true
            }

            /*---------------------------tpg_pixel_clk------------------------------*/
            SysctlClkNodeE::SysctlClkTpgPixel => {
                false
            }

            /*---------------------------disp_pixel_clk------------------------------*/
            SysctlClkNodeE::SysctlClkDisplayPixel => {
                let mut cfg: u32 = 0;
                match parent {
                    SysctlClkNodeE::SysctlClkRootPll2Div4 => {
                        cfg = ((0 << 9) | (1 << 26));
                    }
                    SysctlClkNodeE::SysctlClkRootPll2Div3 => {
                        cfg = ((1 << 9) | (1 << 26));
                    }
                    SysctlClkNodeE::SysctlClkRootPll0 => {
                        cfg = ((2 << 9) | (1 << 26));
                    }
                    SysctlClkNodeE::SysctlClkRootPll1 => {
                        cfg = ((3 << 9) | (1 << 26));
                    }
                    _ => panic!("unsupported leaf node in sysctl_clk_set_leaf_parent!")
                }
                sysctl.0.disp_pixel_clk_cfg.write(|w| unsafe{ w.bits(cfg) });
                true
            }

            /*---------------------------audio_out_serial_clk------------------------------*/
            SysctlClkNodeE::SysctlClkAudioOutSerial => {
                false //always pll1_div3
            }

            /*---------------------------audio_in_serial_clk------------------------------*/
            SysctlClkNodeE::SysctlClkAudioInSerial => {
                false //always pll1_div3
            }

            /*---------------------------sd master/sd 0/1/2 clk------------------------------*/
            SysctlClkNodeE::SysctlClkSdMaster => {
                false //always pll0
            }
            SysctlClkNodeE::SysctlClkSdMasterSd0 => {
                false //always SYSCTL_CLK_SD_MASTER
            }
            SysctlClkNodeE::SysctlClkSdMasterSd1 => {
                false //always SYSCTL_CLK_SD_MASTER
            }
            SysctlClkNodeE::SysctlClkSdMasterSd2 => {
                false //always SYSCTL_CLK_SD_MASTER
            }

            /*---------------------------mipi clocks-----------------------------------------*/
            SysctlClkNodeE::SysctlClkMipiRef => {
                let mut cfg: u32 = 0;
                match parent {
                    SysctlClkNodeE::SysctlClkRootPll2Div2 => {
                        cfg = ((0 << 2) | (1 << 18));
                    }
                    SysctlClkNodeE::SysctlClkRootPll1 => {
                        cfg = ((1 << 2) | (1 << 18));
                    }
                    _ => panic!("unsupported leaf node in sysctl_clk_set_leaf_parent!")
                }
                sysctl.0.txdphy_clk_en.write(|w| unsafe{ w.bits(cfg) });
                true
            }
            SysctlClkNodeE::SysctlClkMipiRefTxphyRef => {
                false
            }
            SysctlClkNodeE::SysctlClkMipiRefRxphyRef => {
                false
            }
            SysctlClkNodeE::SysctlClkMipiRefTxphyPll => {
                false
            }

            /*---------------------------security ahb clocks-----------------------------------------*/
            SysctlClkNodeE::SysctlClkSecHclk => {
                false /* always pll1 div 3*/
            }
            SysctlClkNodeE::SysctlClkPufHclk
            | SysctlClkNodeE::SysctlClkOtpHclk
            | SysctlClkNodeE::SysctlClkRomHclk
            | SysctlClkNodeE::SysctlClkRsaHclk
            | SysctlClkNodeE::SysctlClkAesHclk
            | SysctlClkNodeE::SysctlClkShaHclk => {
                false /* always SYSCTL_CLK_SEC_HCLK*/
            }

            /*---------------------------ddr controller core clocks-----------------------------------*/
            SysctlClkNodeE::SysctlClkDdrControllerCore => {
                let mut cfg: u32 = 0;
                match parent {
                    SysctlClkNodeE::SysctlClkRootPll1 => {
                        cfg = (0 << 0) | (1 << 16);
                    }
                    SysctlClkNodeE::SysctlClkRootPll2Div2 => {
                        cfg = (1 << 0) | (1 << 16);
                    }
                    SysctlClkNodeE::SysctlClkRootPll3Div3 => {
                        cfg = (2 << 0) | (1 << 16);
                    }
                    SysctlClkNodeE::SysctlClkRootPll3Div2 => {
                        cfg = (3 << 0) | (1 << 16);
                    }
                    _ => panic!("unsupported leaf node in sysctl_clk_set_leaf_parent!")
                }
                sysctl.0.mem_ctl_clk_cfg.write(|w| unsafe{ w.bits(cfg) });
                true
            }

            /*---------------------------emac loopback clock------------------------------------------*/
            SysctlClkNodeE::SysctlClkEmacLoopback => {
                false
            }

            /*---------------------------uart system clock--------------------------------------------*/
            SysctlClkNodeE::SysctlClkUart3Sclk => {
                let mut cfg: u32 = 0;
                match parent {
                    SysctlClkNodeE::SysctlClkRootOscIn0 => {
                        cfg = ((0 << 0) | (1 << 24));
                    }
                    SysctlClkNodeE::SysctlClkRootPll0Div4 => {
                        cfg = ((1 << 0) | (1 << 24));
                    }
                    _ => panic!("unsupported leaf node in sysctl_clk_set_leaf_parent!")
                }
                sysctl.0.uart_sclk_cfg[3].write(|w| unsafe{ w.bits(cfg) });
                true
            }
            SysctlClkNodeE::SysctlClkUart2Sclk => {
                let mut cfg: u32 = 0;
                match parent {
                    SysctlClkNodeE::SysctlClkRootOscIn0 => {
                        cfg = ((0 << 0) | (1 << 24));
                    }
                    SysctlClkNodeE::SysctlClkRootPll0Div4 => {
                        cfg = ((1 << 0) | (1 << 24));
                    }
                    _ => panic!("unsupported leaf node in sysctl_clk_set_leaf_parent!")
                }
                sysctl.0.uart_sclk_cfg[2].write(|w| unsafe{ w.bits(cfg) });
                true
            }
            SysctlClkNodeE::SysctlClkUart1Sclk => {
                let mut cfg: u32 = 0;
                match parent {
                    SysctlClkNodeE::SysctlClkRootOscIn0 => {
                        cfg = ((0 << 0) | (1 << 24));
                    }
                    SysctlClkNodeE::SysctlClkRootPll0Div4 => {
                        cfg = ((1 << 0) | (1 << 24));
                    }
                    _ => panic!("unsupported leaf node in sysctl_clk_set_leaf_parent!")
                }
                sysctl.0.uart_sclk_cfg[1].write(|w| unsafe{ w.bits(cfg) });
                true
            }
            SysctlClkNodeE::SysctlClkUart0Sclk => {
                let mut cfg: u32 = 0;
                match parent {
                    SysctlClkNodeE::SysctlClkRootOscIn0 => {
                        cfg = ((0 << 0) | (1 << 24));
                    }
                    SysctlClkNodeE::SysctlClkRootPll0Div4 => {
                        cfg = ((1 << 0) | (1 << 24));
                    }
                    _ => panic!("unsupported leaf node in sysctl_clk_set_leaf_parent!")
                }
                sysctl.0.uart_sclk_cfg[0].write(|w| unsafe{ w.bits(cfg) });
                true
            }

            /*---------------------------spi system clock---------------------------------------------*/
            SysctlClkNodeE::SysctlClkSpi0Sclk => {
                let mut cfg: u32 = 0;
                match parent {
                    SysctlClkNodeE::SysctlClkRootOscIn0 => {
                        cfg = ((0 << 0) | (1 << 24));
                    }
                    SysctlClkNodeE::SysctlClkRootPll0 => {
                        cfg = ((1 << 0) | (1 << 24));
                    }
                    _ => panic!("unsupported leaf node in sysctl_clk_set_leaf_parent!")
                }
                sysctl.0.spi_sclk_cfg[0].write(|w| unsafe{ w.bits(cfg) });
                true
            }
            SysctlClkNodeE::SysctlClkSpi1Sclk => {
                let mut cfg: u32 = 0;
                match parent {
                    SysctlClkNodeE::SysctlClkRootOscIn0 => {
                        cfg = ((0 << 0) | (1 << 24));
                    }
                    SysctlClkNodeE::SysctlClkRootPll0 => {
                        cfg = ((1 << 0) | (1 << 24));
                    }
                    _ => panic!("unsupported leaf node in sysctl_clk_set_leaf_parent!")
                }
                sysctl.0.spi_sclk_cfg[1].write(|w| unsafe{ w.bits(cfg) });
                true
            }
            SysctlClkNodeE::SysctlClkSpi2Sclk => {
                let mut cfg: u32 = 0;
                match parent {
                    SysctlClkNodeE::SysctlClkRootOscIn0 => {
                        cfg = ((0 << 0) | (1 << 24));
                    }
                    SysctlClkNodeE::SysctlClkRootPll0 => {
                        cfg = ((1 << 0) | (1 << 24));
                    }
                    _ => panic!("unsupported leaf node in sysctl_clk_set_leaf_parent!")
                }
                sysctl.0.spi_sclk_cfg[2].write(|w| unsafe{ w.bits(cfg) });
                true
            }
            SysctlClkNodeE::SysctlClkSpi3Sclk => {
                false /*always osc25m*/
            }

            /*---------------------------OTP system clock---------------------------------------------*/
            SysctlClkNodeE::SysctlClkOtpSclk => {
                false
            }

            /*---------------------------i2c system clock---------------------------------------------*/
            SysctlClkNodeE::SysctlClkI2c0Sclk => {
                let mut cfg: u32 = 0;
                match parent {
                    SysctlClkNodeE::SysctlClkRootPll0Div4 => {
                        cfg = ((0 << 9) | (1 << 26));
                    }
                    SysctlClkNodeE::SysctlClkRootOscIn0 => {
                        cfg = ((1 << 9) | (1 << 26));
                    }
                    _ => panic!("unsupported leaf node in sysctl_clk_set_leaf_parent!")
                }
                sysctl.0.i2c_icclk_cfg[0].write(|w| unsafe{ w.bits(cfg) });
                true
            }
            SysctlClkNodeE::SysctlClkI2c1Sclk => {
                let mut cfg: u32 = 0;
                match parent {
                    SysctlClkNodeE::SysctlClkRootPll0Div4 => {
                        cfg = ((0 << 9) | (1 << 26));
                    }
                    SysctlClkNodeE::SysctlClkRootOscIn0 => {
                        cfg = ((1 << 9) | (1 << 26));
                    }
                    _ => panic!("unsupported leaf node in sysctl_clk_set_leaf_parent!")
                }
                sysctl.0.i2c_icclk_cfg[1].write(|w| unsafe{ w.bits(cfg) });
                true
            }
            SysctlClkNodeE::SysctlClkI2c2Sclk => {
                let mut cfg: u32 = 0;
                match parent {
                    SysctlClkNodeE::SysctlClkRootPll0Div4 => {
                        cfg = ((0 << 9) | (1 << 26));
                    }
                    SysctlClkNodeE::SysctlClkRootOscIn0 => {
                        cfg = ((1 << 9) | (1 << 26));
                    }
                    _ => panic!("unsupported leaf node in sysctl_clk_set_leaf_parent!")
                }
                sysctl.0.i2c_icclk_cfg[2].write(|w| unsafe{ w.bits(cfg) });
                true
            }
            SysctlClkNodeE::SysctlClkI2c3Sclk => {
                let mut cfg: u32 = 0;
                match parent {
                    SysctlClkNodeE::SysctlClkRootPll0Div4 => {
                        cfg = ((0 << 9) | (1 << 26));
                    }
                    SysctlClkNodeE::SysctlClkRootOscIn0 => {
                        cfg = ((1 << 9) | (1 << 26));
                    }
                    _ => panic!("unsupported leaf node in sysctl_clk_set_leaf_parent!")
                }
                sysctl.0.i2c_icclk_cfg[3].write(|w| unsafe{ w.bits(cfg) });
                true
            }
            SysctlClkNodeE::SysctlClkI2c4Sclk => {
                let mut cfg: u32 = 0;
                match parent {
                    SysctlClkNodeE::SysctlClkRootPll0Div4 => {
                        cfg = ((0 << 9) | (1 << 26));
                    }
                    SysctlClkNodeE::SysctlClkRootOscIn0 => {
                        cfg = ((1 << 9) | (1 << 26));
                    }
                    _ => panic!("unsupported leaf node in sysctl_clk_set_leaf_parent!")
                }
                sysctl.0.i2c_icclk_cfg[4].write(|w| unsafe{ w.bits(cfg) });
                true
            }
            SysctlClkNodeE::SysctlClkI2c5Sclk => {
                let mut cfg: u32 = 0;
                match parent {
                    SysctlClkNodeE::SysctlClkRootPll0Div4 => {
                        cfg = ((0 << 9) | (1 << 26));
                    }
                    SysctlClkNodeE::SysctlClkRootOscIn0 => {
                        cfg = ((1 << 9) | (1 << 26));
                    }
                    _ => panic!("unsupported leaf node in sysctl_clk_set_leaf_parent!")
                }
                sysctl.0.i2c_icclk_cfg[5].write(|w| unsafe{ w.bits(cfg) });
                true
            }
            SysctlClkNodeE::SysctlClkI2c6Sclk => {
                let mut cfg: u32 = 0;
                match parent {
                    SysctlClkNodeE::SysctlClkRootPll0Div4 => {
                        cfg = ((0 << 9) | (1 << 26));
                    }
                    SysctlClkNodeE::SysctlClkRootOscIn0 => {
                        cfg = ((1 << 9) | (1 << 26));
                    }
                    _ => panic!("unsupported leaf node in sysctl_clk_set_leaf_parent!")
                }
                sysctl.0.i2c_icclk_cfg[6].write(|w| unsafe{ w.bits(cfg) });
                true
            }

            /*---------------------------wdt system clock---------------------------------------------*/
            SysctlClkNodeE::SysctlClkWdt0Tclk
            | SysctlClkNodeE::SysctlClkWdt1Tclk
            | SysctlClkNodeE::SysctlClkWdt2Tclk => {
                false
            }

            /*---------------------------vad system clk-----------------------------------------------*/
            SysctlClkNodeE::SysctlClkVadTclk => {
                false
            }

            /*---------------------------timer0-5 system clock----------------------------------------*/
            SysctlClkNodeE::SysctlClkTimer0Tclk => {
                let mut cfg: u32 = 0;
                match parent {
                    SysctlClkNodeE::SysctlClkRootOscIn1 => {
                        cfg = ((0 << 0) | (1 << 24));
                    }
                    SysctlClkNodeE::SysctlClkRootOscIn0 => {
                        cfg = ((1 << 0) | (1 << 24));
                    }
                    _ => panic!("unsupported leaf node in sysctl_clk_set_leaf_parent!")
                }
                sysctl.0.timer_tclk_src.write(|w| unsafe{ w.bits(cfg) });
                true
            }
            SysctlClkNodeE::SysctlClkTimer1Tclk => {
                let mut cfg: u32 = 0;
                match parent {
                    SysctlClkNodeE::SysctlClkRootOscIn1 => {
                        cfg = ((0 << 4) | (1 << 25));
                    }
                    SysctlClkNodeE::SysctlClkRootOscIn0 => {
                        cfg = ((1 << 4) | (1 << 25));
                    }
                    _ => panic!("unsupported leaf node in sysctl_clk_set_leaf_parent!")
                }
                sysctl.0.timer_tclk_src.write(|w| unsafe{ w.bits(cfg) });
                true
            }
            SysctlClkNodeE::SysctlClkTimer2Tclk => {
                let mut cfg: u32 = 0;
                match parent {
                    SysctlClkNodeE::SysctlClkRootOscIn1 => {
                        cfg = ((0 << 8) | (1 << 26));
                    }
                    SysctlClkNodeE::SysctlClkRootOscIn0 => {
                        cfg = ((1 << 8) | (1 << 26));
                    }
                    _ => panic!("unsupported leaf node in sysctl_clk_set_leaf_parent!")
                }
                sysctl.0.timer_tclk_src.write(|w| unsafe{ w.bits(cfg) });
                true
            }
            SysctlClkNodeE::SysctlClkTimer3Tclk => {
                let mut cfg: u32 = 0;
                match parent {
                    SysctlClkNodeE::SysctlClkRootOscIn1 => {
                        cfg = ((0 << 12) | (1 << 27));
                    }
                    SysctlClkNodeE::SysctlClkRootOscIn0 => {
                        cfg = ((1 << 12) | (1 << 27));
                    }
                    _ => panic!("unsupported leaf node in sysctl_clk_set_leaf_parent!")
                }
                sysctl.0.timer_tclk_src.write(|w| unsafe{ w.bits(cfg) });
                true
            }
            SysctlClkNodeE::SysctlClkTimer4Tclk => {
                let mut cfg: u32 = 0;
                match parent {
                    SysctlClkNodeE::SysctlClkRootOscIn1 => {
                        cfg = ((0 << 16) | (1 << 28));
                    }
                    SysctlClkNodeE::SysctlClkRootOscIn0 => {
                        cfg = ((1 << 16) | (1 << 28));
                    }
                    _ => panic!("unsupported leaf node in sysctl_clk_set_leaf_parent!")
                }
                sysctl.0.timer_tclk_src.write(|w| unsafe{ w.bits(cfg) });
                true
            }
            SysctlClkNodeE::SysctlClkTimer5Tclk => {
                let mut cfg: u32 = 0;
                match parent {
                    SysctlClkNodeE::SysctlClkRootOscIn1 => {
                        cfg = ((0 << 20) | (1 << 29));
                    }
                    SysctlClkNodeE::SysctlClkRootOscIn0 => {
                        cfg = ((1 << 20) | (1 << 29));
                    }
                    _ => panic!("unsupported leaf node in sysctl_clk_set_leaf_parent!")
                }
                sysctl.0.timer_tclk_src.write(|w| unsafe{ w.bits(cfg) });
                true
            }

            /*------------------------------------usb clock-------------------------------------------*/
            SysctlClkNodeE::SysctlClkUsbPhyCore => {
                false
            }
            SysctlClkNodeE::SysctlClkUsbWakeup => {
                false
            }

            _ => panic!("unsupported leaf node in sysctl_clk_set_leaf_parent!")
        }
    }
    else {
        panic!("SYSCTL is Null!")
    }
}

/* 获取时钟树上叶子节点时钟源 */
#[allow(unused)]
pub fn sysctl_clk_get_leaf_parent(leaf: SysctlClkNodeE) -> SysctlClkNodeE {
    // TODO
    SysctlClkNodeE::SysctlClkWdt2Tclk
}

/* 设置时钟节点enable,注意:只设置本时钟节点的enable，不会设置上游时钟的enable。
   同linux kernel的区别: linux kernel clock framework 会自动设置上游时钟的enable，测试代码没有kernel框架，因此只设置本节点时钟的enable */
#[allow(unused)]
pub fn sysctl_clk_set_leaf_en(leaf: SysctlClkNodeE, enable: bool) {
    // TODO
}

/* 获取本时钟节点的enable状态 */
#[allow(unused)]
pub fn sysctl_clk_get_leaf_en(leaf: SysctlClkNodeE) -> bool {
    // TODO
    false
}

/* 获取本时钟节点的分频系数 */
#[allow(unused)]
pub fn sysctl_clk_set_leaf_div(leaf: SysctlClkNodeE, numerator: u32, denominator: u32) -> bool {
    false
}

/* 获取本时钟节点的分频系数 */
#[allow(unused)]
pub fn sysctl_clk_get_leaf_div(leaf: SysctlClkNodeE) -> f64 {
    if let Some(sysctl) = SYSCTL.get(){
        match leaf {

            /*---------------------------AX25MP------------------------------------*/
            SysctlClkNodeE::SysctlClkAx25mSrc => {
                12.0 - (((sysctl.0.ax25m_clk_cfg.read().bits() >> 4) & 0xf) as f64) / 12.0
            }
            SysctlClkNodeE::SysctlClkAx25mCore0
            | SysctlClkNodeE::SysctlClkAx25mCore1
            | SysctlClkNodeE::SysctlClkAx25mCore0Dc
            | SysctlClkNodeE::SysctlClkAx25mCore1Dc
            | SysctlClkNodeE::SysctlClkAx25mMctl => {
                1.0
            }
            SysctlClkNodeE::SysctlClkAx25mMtimer => {
                1.0 / ((((sysctl.0.ax25m_mtimer_clk_cfg.read().bits() >> 2) & 0x1f) + 1) as f64)
            }

            /*---------------------------AX25P------------------------------------*/
            SysctlClkNodeE::SysctlClkAx25pSrc => {
                (6.0 - ((sysctl.0.ax25p_clk_cfg.read().bits() >> 4) & 0x7) as f64) / 6.0
            }
            SysctlClkNodeE::SysctlClkAx25pCore 
            | SysctlClkNodeE::SysctlClkAx25pLm  => {
                1.0
            }
            SysctlClkNodeE::SysctlClkAx25pMtimer => {
                1.0 / (((sysctl.0.ax25p_mtimer_clk_cfg.read().bits() >> 2) & 0x1F) + 1) as f64
            }

            /*---------------------------GNNE------------------------------------*/
            SysctlClkNodeE::SysctlClkGnneSys => {
                (6.0 - ((sysctl.0.gnne_sysclk_cfg.read().bits() >> 4) & 0x7) as f64) / 6.0
            }
            SysctlClkNodeE::SysctlClkGnneAxi => {
                (6.0 - ((sysctl.0.gnne_aclk_cfg.read().bits() >> 4) & 0x7) as f64) / 6.0
            }
            SysctlClkNodeE::SysctlClkGnneAxiNoc => {
                1.0
            }
            SysctlClkNodeE::SysctlClkGnneAxiMctl => {
                1.0
            }

            /*---------------------------NOC0------------------------------------*/
            SysctlClkNodeE::SysctlClkNocClk0 => {
                (6.0 - ((sysctl.0.noc_clk_cfg.read().bits() >> 4) & 0x7) as f64) / 6.0
            }
            SysctlClkNodeE::SysctlClkNocClk0Div2 => {
                2.0
            }
            SysctlClkNodeE::SysctlClkNocClk0Div3 => {
                3.0
            }
            SysctlClkNodeE::SysctlClkNocClk0Div4 => {
                4.0
            }
            SysctlClkNodeE::SysctlClkNocClk0PeriDmaAxi
            | SysctlClkNodeE::SysctlClkNocClk0SysDmaAxi
            | SysctlClkNodeE::SysctlClkNocClk0Sram0Axi
            | SysctlClkNodeE::SysctlClkNocClk0Sram1Axi
            | SysctlClkNodeE::SysctlClkNocClk0AxiP3 => {
                1.0
            }
            SysctlClkNodeE::SysctlClkNocClk0Div2MctlAhb
            | SysctlClkNodeE::SysctlClkNocClk0Div2UsbAhb
            | SysctlClkNodeE::SysctlClkNocClk0Div2SdSlvAhb => {
                1.0
            }
            SysctlClkNodeE::SysctlClkNocClk0Div3Sd0Ahb
            | SysctlClkNodeE::SysctlClkNocClk0Div3Sd1Ahb
            | SysctlClkNodeE::SysctlClkNocClk0Div3Sd2Ahb
            | SysctlClkNodeE::SysctlClkNocClk0Div3EmacAhb
            | SysctlClkNodeE::SysctlClkNocClk0Div3PeriDmaApb
            | SysctlClkNodeE::SysctlClkNocClk0Div3SysDmaApb => {
                1.0
            }
            SysctlClkNodeE::SysctlClkNocClk0Div4Wdt0Apb
            | SysctlClkNodeE::SysctlClkNocClk0Div4Wdt1Apb
            | SysctlClkNodeE::SysctlClkNocClk0Div4Wdt2Apb
            | SysctlClkNodeE::SysctlClkNocClk0Div4TimerApb
            | SysctlClkNodeE::SysctlClkNocClk0Div4RtcApb
            | SysctlClkNodeE::SysctlClkNocClk0Div4GpioApb
            | SysctlClkNodeE::SysctlClkNocClk0Div4IomuxApb
            | SysctlClkNodeE::SysctlClkNocClk0Div4I2c0Apb
            | SysctlClkNodeE::SysctlClkNocClk0Div4I2c1Apb
            | SysctlClkNodeE::SysctlClkNocClk0Div4I2c2Apb
            | SysctlClkNodeE::SysctlClkNocClk0Div4I2c3Apb
            | SysctlClkNodeE::SysctlClkNocClk0Div4I2c4Apb
            | SysctlClkNodeE::SysctlClkNocClk0Div4I2c5Apb
            | SysctlClkNodeE::SysctlClkNocClk0Div4I2c6Apb
            | SysctlClkNodeE::SysctlClkNocClk0Div4PwmApb
            | SysctlClkNodeE::SysctlClkNocClk0Div4MailboxApb
            | SysctlClkNodeE::SysctlClkNocClk0Div4VadApb => {
                1.0
            }

            /*---------------------------NOC1------------------------------------*/
            SysctlClkNodeE::SysctlClkNocClk1 => {
                (6.0 - ((sysctl.0.noc_clk_cfg.read().bits() >> 8) & 0x7) as f64) / 6.0
            }

            SysctlClkNodeE::SysctlClkNocClk1AxiMctl
            | SysctlClkNodeE::SysctlClkNocClk1H264Axi
            | SysctlClkNodeE::SysctlClkNocClk1VoAxi
            | SysctlClkNodeE::SysctlClkNocClk1TwodAxi
            | SysctlClkNodeE::SysctlClkNocClk1MfbcAxi
            | SysctlClkNodeE::SysctlClkNocClk1ViAxi
            | SysctlClkNodeE::SysctlClkNocClk1IspF2kAxi
            | SysctlClkNodeE::SysctlClkNocClk1IspR2kAxi
            | SysctlClkNodeE::SysctlClkNocClk1IspTofAxi

            | SysctlClkNodeE::SysctlClkNocClk1PeriApb

            | SysctlClkNodeE::SysctlClkNocClk1PeriApbUart0Apb
            | SysctlClkNodeE::SysctlClkNocClk1PeriApbUart1Apb
            | SysctlClkNodeE::SysctlClkNocClk1PeriApbUart2Apb
            | SysctlClkNodeE::SysctlClkNocClk1PeriApbUart3Apb
            | SysctlClkNodeE::SysctlClkNocClk1PeriApbAudioApb
            
            | SysctlClkNodeE::SysctlClkNocClk1PeriAhb
            | SysctlClkNodeE::SysctlClkNocClk1PeriAhbSpi0Ahb
            | SysctlClkNodeE::SysctlClkNocClk1PeriAhbSpi1Ahb
            | SysctlClkNodeE::SysctlClkNocClk1PeriAhbSpi2Ahb
            | SysctlClkNodeE::SysctlClkNocClk1PeriAhbSpi3Ahb => {
                1.0
            }

            SysctlClkNodeE::SysctlClkNocClk1Div4 => {
                4.0
            }

            SysctlClkNodeE::SysctlClkNocClk1Csi0Apb
            | SysctlClkNodeE::SysctlClkNocClk1Csi1Apb
            | SysctlClkNodeE::SysctlClkNocClk1Csi2Apb
            | SysctlClkNodeE::SysctlClkNocClk1Csi3Apb
            | SysctlClkNodeE::SysctlClkNocClk1F2kApb
            | SysctlClkNodeE::SysctlClkNocClk1R2kApb
            | SysctlClkNodeE::SysctlClkNocClk1TofApb
            | SysctlClkNodeE::SysctlClkNocClk1MfbcApb
            | SysctlClkNodeE::SysctlClkNocClk1MipiCornerApb
            | SysctlClkNodeE::SysctlClkNocClk1ViApb => {
                1.0
            }

            /*---------------------------DISPLAY------------------------------------*/
            SysctlClkNodeE::SysctlClkDispSysAndApbClkDiv => {
                1.0 / ((((sysctl.0.disp_sys_pclk_en.read().bits() >> 4) & 0xf) + 1) as f64)
            }
            SysctlClkNodeE::SysctlClkDispSysAndApbClkDivDsiApb
            | SysctlClkNodeE::SysctlClkDispSysAndApbClkDivDsiSystem
            | SysctlClkNodeE::SysctlClkDispSysAndApbClkDivVoApb
            | SysctlClkNodeE::SysctlClkDispSysAndApbClkDivTwodApb
            | SysctlClkNodeE::SysctlClkDispSysAndApbClkDivBt1120Apb => {
                1.0
            }
            
            /*---------------------------csi0_system_clk-----------------------------*/
            SysctlClkNodeE::SysctlClkCsi0System => {
                1.0 / ((((sysctl.0.csi_sys_clk_cfg[0].read().bits() >> 0) & 0xf) + 1) as f64)
            }

            /*---------------------------csi1_system_clk-----------------------------*/
            SysctlClkNodeE::SysctlClkCsi1System => {
                1.0 / ((((sysctl.0.csi_sys_clk_cfg[1].read().bits() >> 0) & 0xf) + 1) as f64)
            }

            /*---------------------------csi0_pixel_clk------------------------------*/
            SysctlClkNodeE::SysctlClkCsi0Pixel => {
                1.0 / ((((sysctl.0.csi_pixel_clk_cfg[0].read().bits() >> 0) & 0x7f) + 1) as f64)
            }

            /*---------------------------csi1_pixel_clk------------------------------*/
            SysctlClkNodeE::SysctlClkCsi1Pixel => {
                1.0 / ((((sysctl.0.csi_pixel_clk_cfg[1].read().bits() >> 0) & 0x7f) + 1) as f64)
            }

            /*---------------------------tpg_pixel_clk------------------------------*/
            SysctlClkNodeE::SysctlClkTpgPixel => {
                1.0 / ((((sysctl.0.tpg_pixel_clk_cfg.read().bits() >> 0) & 0x7f) + 1) as f64)
            }

            /*---------------------------disp_pixel_clk------------------------------*/
            SysctlClkNodeE::SysctlClkDisplayPixel => {
                1.0 / ((((sysctl.0.disp_pixel_clk_cfg.read().bits() >> 0) & 0x7f) + 1) as f64)
            }

            /*---------------------------audio_out_serial_clk------------------------------*/
            SysctlClkNodeE::SysctlClkAudioOutSerial => {
                ((sysctl.0.audif_sclk_cfg.read().bits() >> 12) & 0xff) as f64 / ((sysctl.0.audif_sclk_cfg.read().bits() >> 0) & 0xFFF) as f64
            }

            /*---------------------------audio_in_serial_clk------------------------------*/
            SysctlClkNodeE::SysctlClkAudioInSerial => {
                ((sysctl.0.audif_devclk_cfg.read().bits() >> 12) & 0xff) as f64 / ((sysctl.0.audif_devclk_cfg.read().bits() >> 0) & 0xFFF) as f64
            }

            /*---------------------------sd master/sd 0/1/2 clk------------------------------*/
            SysctlClkNodeE::SysctlClkSdMaster => {
                5.0
            }
            SysctlClkNodeE::SysctlClkSdMasterSd0 => {
                1.0
            }
            SysctlClkNodeE::SysctlClkSdMasterSd1 => {
                1.0
            }
            SysctlClkNodeE::SysctlClkSdMasterSd2 => {
                1.0
            }

            /*---------------------------mipi clocks-----------------------------------------*/
            SysctlClkNodeE::SysctlClkMipiRef => {
                1.0 / ((((sysctl.0.txdphy_clk_en.read().bits() >> 4) & 0x7f) + 1) as f64)
            }
            SysctlClkNodeE::SysctlClkMipiRefTxphyRef => {
                1.0
            }
            SysctlClkNodeE::SysctlClkMipiRefRxphyRef => {
                1.0
            }
            SysctlClkNodeE::SysctlClkMipiRefTxphyPll => {
                1.0
            }

            /*---------------------------security ahb clocks-----------------------------------------*/
            SysctlClkNodeE::SysctlClkSecHclk => {
                (8.0 - ((sysctl.0.sec_sys_bus_clk_cfg.read().bits() >> 0) & 0x7) as f64) / 8.0
            }
            SysctlClkNodeE::SysctlClkPufHclk
            | SysctlClkNodeE::SysctlClkOtpHclk
            | SysctlClkNodeE::SysctlClkRomHclk
            | SysctlClkNodeE::SysctlClkRsaHclk
            | SysctlClkNodeE::SysctlClkAesHclk
            | SysctlClkNodeE::SysctlClkShaHclk => {
                1.0
            }

            /*---------------------------ddr controller core clocks-----------------------------------*/
            SysctlClkNodeE::SysctlClkDdrControllerCore => {
                1.0 / ((((sysctl.0.mem_ctl_clk_cfg.read().bits() >> 4) & 0x3) + 1) as f64)
            }

            /*---------------------------emac loopback clock------------------------------------------*/
            SysctlClkNodeE::SysctlClkEmacLoopback => {
                1.0 / ((((sysctl.0.emac_trx_clk_cfg.read().bits() >> 10) & 0x3f) + 1) as f64)
            }

            /*---------------------------uart system clock--------------------------------------------*/
            SysctlClkNodeE::SysctlClkUart0Sclk => {
                1.0 / ((((sysctl.0.uart_sclk_cfg[0].read().bits() >> 4) & 0x1f) + 1) as f64)
            }
            SysctlClkNodeE::SysctlClkUart1Sclk => {
                1.0 / ((((sysctl.0.uart_sclk_cfg[1].read().bits() >> 4) & 0x1f) + 1) as f64)
            }
            SysctlClkNodeE::SysctlClkUart2Sclk => {
                1.0 / ((((sysctl.0.uart_sclk_cfg[2].read().bits() >> 4) & 0x1f) + 1) as f64)
            }
            SysctlClkNodeE::SysctlClkUart3Sclk => {
                1.0 / ((((sysctl.0.uart_sclk_cfg[3].read().bits() >> 4) & 0x1f) + 1) as f64)
            }

            /*---------------------------spi system clock---------------------------------------------*/
            SysctlClkNodeE::SysctlClkSpi0Sclk => {
                1.0 / ((((sysctl.0.spi_sclk_cfg[0].read().bits() >> 4) & 0xf) + 1) as f64)
            }
            SysctlClkNodeE::SysctlClkSpi1Sclk => {
                1.0 / ((((sysctl.0.spi_sclk_cfg[1].read().bits() >> 4) & 0xf) + 1) as f64)
            }
            SysctlClkNodeE::SysctlClkSpi2Sclk => {
                1.0 / ((((sysctl.0.spi_sclk_cfg[2].read().bits() >> 4) & 0xf) + 1) as f64)
            }
            SysctlClkNodeE::SysctlClkSpi3Sclk => {
                1.0 / ((((sysctl.0.spi_sclk_cfg[3].read().bits() >> 4) & 0xf) + 1) as f64)
            }
            
            /*---------------------------OTP system clock---------------------------------------------*/
            SysctlClkNodeE::SysctlClkOtpSclk => {
                2.0 /* hareware div ,cannot be set by software. 25M/2=12.5M*/
            }

            /*---------------------------i2c system clock---------------------------------------------*/
            SysctlClkNodeE::SysctlClkI2c0Sclk => {
                1.0 / ((((sysctl.0.i2c_icclk_cfg[0].read().bits() >> 0) & 0x3f) + 1) as f64)
            }
            SysctlClkNodeE::SysctlClkI2c1Sclk => {
                1.0 / ((((sysctl.0.i2c_icclk_cfg[1].read().bits() >> 0) & 0x3f) + 1) as f64)
            }
            SysctlClkNodeE::SysctlClkI2c2Sclk => {
                1.0 / ((((sysctl.0.i2c_icclk_cfg[2].read().bits() >> 0) & 0x3f) + 1) as f64)
            }
            SysctlClkNodeE::SysctlClkI2c3Sclk => {
                1.0 / ((((sysctl.0.i2c_icclk_cfg[3].read().bits() >> 0) & 0x3f) + 1) as f64)
            }
            SysctlClkNodeE::SysctlClkI2c4Sclk => {
                1.0 / ((((sysctl.0.i2c_icclk_cfg[4].read().bits() >> 0) & 0x3f) + 1) as f64)
            }
            SysctlClkNodeE::SysctlClkI2c5Sclk => {
                1.0 / ((((sysctl.0.i2c_icclk_cfg[5].read().bits() >> 0) & 0x3f) + 1) as f64)
            }
            SysctlClkNodeE::SysctlClkI2c6Sclk => {
                1.0 / ((((sysctl.0.i2c_icclk_cfg[6].read().bits() >> 0) & 0x3f) + 1) as f64)
            }

            /*---------------------------wdt system clock---------------------------------------------*/
            SysctlClkNodeE::SysctlClkWdt0Tclk => {
                1.0 / ((((sysctl.0.wdt_tclk_cfg[0].read().bits() >> 0) & 0x3f) + 1) as f64)
            }
            SysctlClkNodeE::SysctlClkWdt1Tclk => {
                1.0 / ((((sysctl.0.wdt_tclk_cfg[1].read().bits() >> 0) & 0x3f) + 1) as f64)
            }
            SysctlClkNodeE::SysctlClkWdt2Tclk => {
                1.0 / ((((sysctl.0.wdt_tclk_cfg[2].read().bits() >> 0) & 0x3f) + 1) as f64)
            }

            /*---------------------------vad system clk-----------------------------------------------*/
            SysctlClkNodeE::SysctlClkVadTclk => {
                ((sysctl.0.vad_sclk_cfg.read().bits() >> 12) & 0x3f) as f64 / ((sysctl.0.vad_sclk_cfg.read().bits() >> 0) & 0xFFF) as f64
            }

            /*---------------------------timer0-5 system clock----------------------------------------*/
            SysctlClkNodeE::SysctlClkTimer0Tclk => {
                1.0 / ((((sysctl.0.timer_tclk_cfg.read().bits() >> 0) & 0x3f) + 1) as f64)
            }
            SysctlClkNodeE::SysctlClkTimer1Tclk => {
                1.0 / ((((sysctl.0.timer_tclk_cfg.read().bits() >> 6) & 0x3f) + 1) as f64)
            }
            SysctlClkNodeE::SysctlClkTimer2Tclk => {
                1.0 / ((((sysctl.0.timer_tclk_cfg.read().bits() >> 12) & 0x3f) + 1) as f64)
            }
            SysctlClkNodeE::SysctlClkTimer3Tclk => {
                1.0 / ((((sysctl.0.timer_tclk_cfg.read().bits() >> 18) & 0x3f) + 1) as f64)
            }
            SysctlClkNodeE::SysctlClkTimer4Tclk => {
                1.0 / ((((sysctl.0.timer_tclk_cfg1.read().bits() >> 0) & 0x3f) + 1) as f64)
            }
            SysctlClkNodeE::SysctlClkTimer5Tclk => {
                1.0 / ((((sysctl.0.timer_tclk_cfg1.read().bits() >> 6) & 0x3f) + 1) as f64)
            }

            /*------------------------------------usb clock-------------------------------------------*/
            SysctlClkNodeE::SysctlClkUsbPhyCore => {
                1.0
            }
            SysctlClkNodeE::SysctlClkUsbWakeup => {
                1.0
            }

            _ => panic!("unsupported leaf node in sysctl_clk_get_leaf_div!")
        }
    }
    else {
        panic!("SYSCTL is Null!")
    }
}

/* 设置本时钟节点的相位 */
#[allow(unused)]
pub fn sysctl_clk_set_phase(leaf: SysctlClkNodeE, degree: u32) -> bool {
    let cfg_0: u32 = (0 << 21) | (1 << 26);
    let cfg_180: u32 = (1 << 21) | (1 << 26);
    if let Some(sysctl) = SYSCTL.get() {
        match leaf {
            SysctlClkNodeE::SysctlClkAudioOutSerial => {
                if 0 == degree {
                    sysctl.0.audif_sclk_cfg.write(|w| unsafe{ w.bits(cfg_0) });
                    true
                }
                else if 180 == degree {
                    sysctl.0.audif_sclk_cfg.write(|w| unsafe{ w.bits(cfg_180) });
                    true
                }
                else {
                    false
                }
                
            }
            SysctlClkNodeE::SysctlClkAudioInSerial => {
                if 0 == degree {
                    sysctl.0.audif_devclk_cfg.write(|w| unsafe{ w.bits(cfg_0) });
                    true
                }
                else if 180 == degree {
                    sysctl.0.audif_devclk_cfg.write(|w| unsafe{ w.bits(cfg_180) });
                    true
                }
                else {
                    false
                }
            }
            SysctlClkNodeE::SysctlClkVadTclk => {
                if 0 == degree {
                    sysctl.0.vad_sclk_cfg.write(|w| unsafe{ w.bits((0 << 20) | (1 << 27)) });
                    true
                }
                else if 180 == degree {
                    sysctl.0.vad_sclk_cfg.write(|w| unsafe{ w.bits((1 << 20) | (1 << 27)) });
                    true
                }
                else {
                    false
                }
            }
            _ => panic!("unsupported leaf node in sysctl_clk_set_phase!")
        }
    }
    else {
        panic!("SYSCTL is Null!")
    }
}

/* 获取本时钟节点的相位 */
#[allow(unused)]
pub fn sysctl_clk_get_phase(leaf: SysctlClkNodeE) -> u32 {
    if let Some(sysctl) = SYSCTL.get() {
        match leaf {
            SysctlClkNodeE::SysctlClkAudioOutSerial => {
                if (sysctl.0.audif_sclk_cfg.read().bits() >> 21) & 1 == 0{
                    0
                }
                else {
                    180
                }
            }
            SysctlClkNodeE::SysctlClkAudioInSerial => {
                if (sysctl.0.audif_devclk_cfg.read().bits() >> 21) & 1 == 0{
                    0
                }
                else {
                    180
                }
            }
            SysctlClkNodeE::SysctlClkVadTclk => {
                if (sysctl.0.vad_sclk_cfg.read().bits() >> 20) & 1 == 0{
                    0
                }
                else {
                    180
                }
            }
            _ => panic!("unsupported leaf node in sysctl_clk_get_phase!")
        }
    }
    else {
        panic!("SYSCTL is Null!")
    }
}

/* calc clock freqency */
/* 计算当前时钟节点的频率, 这个API会搜索整个时钟路径，从时钟源开始计算每一级的分频，最终得出当前时钟频率 */
#[allow(unused)]
pub fn sysctl_clk_get_leaf_freq(leaf: SysctlClkNodeE) -> u32 {
    
    0
}


/* 辅助计算函数，本函数会根据父节点时钟/实际需要输出的时钟/分频配置方法 计算出最合适的分频系数 */
#[allow(unused)]
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
    match method {
        SysctlClkMulDivMethordE::SysctlClkMulChangeable => {
            let perfect_divide: i64 = (parent_rate*1000 / rate) as i64;
            let mut abs_min: i64 = i64::abs(perfect_divide - (div_max as i64)*1000 / (mul_min as i64));
            for i in mul_min+1 ..= mul_max {
                let abs_current = i64::abs(perfect_divide - (div_max as i64)*1000 / (i as i64));
                if abs_min > abs_current {
                    abs_min = abs_current;
                    unsafe{ mul.write_volatile(i) }
                }
            }
            unsafe { div.write_volatile(div_min) };
        }
        SysctlClkMulDivMethordE::SysctlClkDivChangeable => {
            let perfect_divide: i64 = (parent_rate*1000 / rate) as i64;
            let mut abs_min: i64 = i64::abs(perfect_divide - (div_min as i64)*1000 / (mul_max as i64));
            unsafe { div.write_volatile(div_min) };

            for i in div_min+1 ..= div_max {
                let abs_current = i64::abs(perfect_divide - (i as i64)*1000 / (mul_max as i64));
                if abs_min > abs_current {
                    abs_min = abs_current;
                    unsafe{ div.write_volatile(i) }
                }
            }
            unsafe{ mul.write_volatile(mul_min) };
        }
        SysctlClkMulDivMethordE::SysctlClkMulDivChangeable => {
            let perfect_divide = parent_rate / rate;
            /*div/mul must > 4 */
            if perfect_divide > ((div_max as i64) / (mul_min as i64)) as u64 || perfect_divide < 4 {
                return -1;
            }

            /* calc greatest common divisor */
            let mut a = rate;
            let mut b = parent_rate;

            while a != b {
                if a > b{
                    a -= b;
                }
                else {
                    b -= a;
                }
            }

            unsafe {
                div.write_volatile((parent_rate / a) as u32);
                div.write_volatile((rate / b) as u32);
            }

            /* calc mul 2^n */
            let mut n = 0;
            let mut i = 0;
            loop {
                if (mul_max >> i) > 0 {
                    n += 1;
                    i += 1;
                }
                else {
                    break;
                }
            }
            n += 1;

            let mut div_ulong: u64 = 
            unsafe{ div.read_volatile() as u64 } * (2 as u64).pow(n) / unsafe{ mul.read_volatile() as u64 };
            let mut mul_ulong: u64 = (2 as u64).pow(n);
            
            while (div_ulong > div_max as u64) || (mul_ulong > mul_max as u64) {
                div_ulong >>= 1;
                mul_ulong >>= 1;
            }

            unsafe {
                div.write_volatile(div_ulong as u32);
                mul.write_volatile(mul_ulong as u32);
                if div.read_volatile() < div_min || mul.read_volatile() < mul_min {
                    return -2;
                }
            }
        }
    }
    0
}

#[allow(unused)]
pub fn sysctl_boot_get_boot_mode() -> SysctlBootModeE {
    // TODO
    SysctlBootModeE::SysctlBootMax
}