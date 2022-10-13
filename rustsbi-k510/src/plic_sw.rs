use core::cell::UnsafeCell;
use rustsbi::{spec::binary::SbiRet, HartMask, Ipi};
use crate::cpu::hart_id;
use constants::*;

/// REFERENCE: https://github.com/riscv-software-src/opensbi/blob/bd355213bfbb209c047e8cc0df56936f6705477f/platform/andes/ae350/plicsw.c
pub(crate) struct PlicSW;

impl Ipi for PlicSW {
    #[inline]
    fn send_ipi_many(&self, hart_mask: HartMask) -> SbiRet {
        let hsm = crate::HSM.wait();
        for i in 0..crate::NUM_HART_MAX {
            if hart_mask.has_bit(i) && hsm.is_ipi_allowed(i) {
                send_ipi(i);
            }
        }
        SbiRet::ok(0)
    }
}

static mut BASE: UnsafeCell<usize> = UnsafeCell::new(0);

mod constants {
    pub(crate) const PLICSW_PENDING_BIT_VEC_OFFSET: usize = 0x1000;
    pub(crate) const PLICSW_ENABLE_REGS_OFFSET: usize = 0x2000;
    pub(crate) const PLICSW_CONTEXT_OFFSET: usize = 0x20_0000;
    pub(crate) const PLICSW_CONTEXT_CLAIM_REGS_OFFSET: usize = PLICSW_CONTEXT_OFFSET + 0x4;

    pub(crate) const PLICSW_PENDING_PER_HART: u32 = 0x8;
    pub(crate) const PLICSW_CONTEXT_PER_HART: usize = 0x8;
}

#[inline]
pub(crate) fn init(base: usize) {
    unsafe { *BASE.get() = base };
}

pub(crate) fn enable_ipi(hart_id: usize) {
    const ENABLE_HART_IPI: u32 = 0x80808080;

    let base = unsafe { *BASE.get() } + PLICSW_ENABLE_REGS_OFFSET;
    let enable_mark: u32 = ENABLE_HART_IPI >> hart_id;
    let enable_reg_address: usize = base + 0x80 * hart_id;
    unsafe {
        (enable_reg_address as *mut u32).write_volatile(enable_mark)
    }
}

#[inline]
pub(crate) fn send_ipi(target_hart_id: usize) {
    // 看起来PLIC_SW把每4个核心划分为一个组，这四个核心可以相互发ipi。
    // 但是超过4个核心的部分呢？
    // 不过k510就俩大核心 + 1dsp核，不用担心这个问题。
    let source_hart_id: usize = hart_id();
    let base = unsafe { *BASE.get() } + PLICSW_PENDING_BIT_VEC_OFFSET + ((source_hart_id / 4) * 4);

    let target_offset: u32 = (PLICSW_PENDING_PER_HART - 1) - (target_hart_id as u32);
    let per_hart_offset: u32 = PLICSW_PENDING_PER_HART * (source_hart_id as u32);
    let val: u32 = 1 << target_offset << per_hart_offset;

    unsafe {
        (base as *mut u32).write_volatile(val);
    }
}

#[inline]
pub(crate) fn clear_ipi() {
    // CLAIM/COMPLETE寄存器通过读写操作判断中断完成情况
    let self_hart_id: usize = hart_id();
    let base = unsafe { *BASE.get() } + PLICSW_CONTEXT_CLAIM_REGS_OFFSET + self_hart_id * PLICSW_CONTEXT_PER_HART;
    let ipi_source_hart_id: u32 = unsafe { (base as *mut u32).read_volatile() };
    unsafe {
        (base as *mut u32).write_volatile(ipi_source_hart_id);
    }
}