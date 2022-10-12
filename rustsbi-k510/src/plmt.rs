use core::cell::UnsafeCell;
use rustsbi::{spec::binary::SbiRet, HartMask, Ipi, Timer};

pub(crate) struct Plmt; //andes Platform-Level Machine Timer

impl Timer for Plmt {
    #[inline]
    fn set_timer(&self, time_value: u64) {
        unsafe {
            riscv::register::mip::clear_stimer();
            mtimecmp::set(time_value);
        }
    }
}

static mut BASE: UnsafeCell<usize> = UnsafeCell::new(0);

mod constants {
    /// mtime 偏移
    pub(crate) const MTIME_OFFSET: usize = 0x0;
    /// mtimecmp_0起始偏移
    pub(crate) const MTIMECMP_0_OFFSET: usize = 0x8;
}

#[inline]
pub(crate) fn init(base: usize) {
    unsafe { *BASE.get() = base };
}

#[allow(unused)]
pub mod mtime {
    use crate::plmt::{
        BASE,
        constants::MTIME_OFFSET
    };

    #[inline]
    pub fn read() -> u64 {
        unsafe { ((BASE.get().read_volatile() + MTIME_OFFSET) as *mut u64).read_volatile() }
    }
}

pub mod mtimecmp {
    use crate::plmt::{
        BASE,
        constants::MTIMECMP_0_OFFSET
    };
    #[naked]
    pub unsafe extern "C" fn set_naked(time_value: u64) {
        core::arch::asm!(
        // 保存必要寄存器
        "   addi sp, sp, -16
                sd   t0, 0(sp)
                sd   t1, 8(sp)
            ",
        // 定位并设置当前核的 mtimecmp
        "   li   t1, {mtimecmp_base}
                la   t0, {base}
                ld   t0, 0(t0)
                add  t0, t0, t1
                csrr t1, mhartid
                slli t1, t1, 3
                add  t0, t0, t1
                sd   a0, 0(t0)
            ",
        // 恢复上下文并返回
        "   ld   t1, 8(sp)
                ld   t0, 0(sp)
                addi sp, sp,  16
                ret
            ",
        mtimecmp_base = MTIMECMP_0_OFFSET
        base = sym BASE,
        options(noreturn)
        )
    }

    #[inline]
    pub fn set(time_value: u64) {
        unsafe { set_naked(time_value) };
    }

    #[inline]
    pub fn clear() {
        unsafe { set_naked(u64::MAX) };
    }
}