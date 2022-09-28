//! A test kernel to test RustSBI function on all platforms

#![feature(naked_functions, asm_sym, asm_const)]
#![no_std]
#![no_main]

use core::arch::asm;
use riscv::register::{
    scause::{Interrupt, Trap},
    sepc,
    stvec::{self, TrapMode},
};

#[macro_use]
mod console;
mod test;

mod constants {
    pub(crate) const LEN_PAGE: usize = 4096; // 4KiB
    pub(crate) const PER_HART_STACK_SIZE: usize = 4 * LEN_PAGE; // 16KiB
    pub(crate) const MAX_HART_NUMBER: usize = 8; // assume 8 cores in QEMU
    pub(crate) const STACK_SIZE: usize = PER_HART_STACK_SIZE * MAX_HART_NUMBER;
}

use constants::*;

#[cfg_attr(not(test), panic_handler)]
fn panic(info: &core::panic::PanicInfo) -> ! {
    use sbi_rt::{system_reset, RESET_REASON_SYSTEM_FAILURE, RESET_TYPE_SHUTDOWN};

    let (hard_id, pc): (usize, usize);
    unsafe { asm!("mv    {}, tp", out(reg) hard_id) };
    unsafe { asm!("auipc {},  0", out(reg) pc) };
    println!("[test-kernel-panic] hart {hard_id} {info}");
    println!("[test-kernel-panic] pc = {pc:#x}");
    println!("[test-kernel-panic] SBI test FAILED due to panic");
    system_reset(RESET_TYPE_SHUTDOWN, RESET_REASON_SYSTEM_FAILURE);
    loop {}
}

/// 内核入口。
///
/// # Safety
///
/// 裸函数。
#[naked]
#[no_mangle]
#[link_section = ".text.entry"]
unsafe extern "C" fn _start(hartid: usize, device_tree_paddr: usize) -> ! {
    asm!(
        "csrw sie, zero",      // 关中断
        "call {select_stack}", // 设置启动栈
        "j    {main}",         // 进入 rust
        select_stack = sym select_stack,
        main = sym primary_rust_main,
        options(noreturn)
    )
}

/// 为每个核记录一个预期的陷入原因，实现陷入代理测试。
/// 总是可以安全地使用，因为这是（硬件）线程独立变量。
static mut EXPECTED: [Option<Trap>; 8] = [None; 8];

extern "C" fn primary_rust_main(hartid: usize, dtb_pa: usize) -> ! {
    zero_bss();

    let BoardInfo { smp, uart } = parse_smp(dtb_pa);
    console::init(uart);
    println!(
        r"
 _____         _     _  __                    _
|_   _|__  ___| |_  | |/ /___ _ __ _ __   ___| |
  | |/ _ \/ __| __| | ' // _ \ '__| '_ \ / _ \ |
  | |  __/\__ \ |_  | . \  __/ |  | | | |  __/ |
  |_|\___||___/\__| |_|\_\___|_|  |_| |_|\___|_|
================================================
| boot hart id          | {hartid:20} |
| smp                   | {smp:20} |
| dtb physical address  | {dtb_pa:#20x} |
------------------------------------------------"
    );

    test::base_extension();
    test::sbi_ins_emulation();

    unsafe { stvec::write(start_trap as usize, TrapMode::Direct) };
    test::trap_execption_delegate(hartid);

    unsafe { riscv::register::sstatus::set_sie() };
    test::trap_interrupt_delegate(hartid);

    test::hsm(hartid, smp);

    sbi_rt::system_reset(sbi_rt::RESET_TYPE_SHUTDOWN, sbi_rt::RESET_REASON_NO_REASON);
    unreachable!()
}

extern "C" fn rust_trap_exception(trap_frame: &mut TrapFrame) {
    use riscv::register::scause;

    let cause = scause::read().cause();
    let expected = unsafe { core::mem::take(&mut EXPECTED[trap_frame.tp]) };
    if Some(cause) == expected {
        match cause {
            Trap::Exception(_) => {
                sepc::write(sepc::read().wrapping_add(4));
            }
            Trap::Interrupt(Interrupt::SupervisorTimer) => {
                sbi_rt::set_timer(u64::MAX);
            }
            _ => {}
        }
    } else {
        panic!("[test-kernel] SBI test FAILED due to unexpected trap {cause:?}");
    }
}

#[cfg(target_pointer_width = "128")]
macro_rules! define_store_load {
    () => {
        ".altmacro
        .macro STORE reg, offset
            sq  \\reg, \\offset* {REGBYTES} (sp)
        .endm
        .macro LOAD reg, offset
            lq  \\reg, \\offset* {REGBYTES} (sp)
        .endm"
    };
}

#[cfg(target_pointer_width = "64")]
macro_rules! define_store_load {
    () => {
        ".altmacro
        .macro STORE reg, offset
            sd  \\reg, \\offset* {REGBYTES} (sp)
        .endm
        .macro LOAD reg, offset
            ld  \\reg, \\offset* {REGBYTES} (sp)
        .endm"
    };
}

#[cfg(target_pointer_width = "32")]
macro_rules! define_store_load {
    () => {
        ".altmacro
        .macro STORE reg, offset
            sw  \\reg, \\offset* {REGBYTES} (sp)
        .endm
        .macro LOAD reg, offset
            lw  \\reg, \\offset* {REGBYTES} (sp)
        .endm"
    };
}

#[naked]
#[link_section = ".text.trap_handler"]
unsafe extern "C" fn start_trap() {
    asm!(define_store_load!(), "
    addi    sp, sp, -17 * {REGBYTES}
    STORE   ra, 0
    STORE   t0, 1
    STORE   t1, 2
    STORE   t2, 3
    STORE   t3, 4
    STORE   t4, 5
    STORE   t5, 6
    STORE   t6, 7
    STORE   a0, 8
    STORE   a1, 9
    STORE   a2, 10
    STORE   a3, 11
    STORE   a4, 12
    STORE   a5, 13
    STORE   a6, 14
    STORE   a7, 15
    STORE   tp, 16
    mv      a0, sp
    call    {rust_trap_exception}
    LOAD    ra, 0
    LOAD    t0, 1
    LOAD    t1, 2
    LOAD    t2, 3
    LOAD    t3, 4
    LOAD    t4, 5
    LOAD    t5, 6
    LOAD    t6, 7
    LOAD    a0, 8
    LOAD    a1, 9
    LOAD    a2, 10
    LOAD    a3, 11
    LOAD    a4, 12
    LOAD    a5, 13
    LOAD    a6, 14
    LOAD    a7, 15
    LOAD    tp, 16
    addi    sp, sp, 17 * {REGBYTES}
    sret
    ",
    REGBYTES = const core::mem::size_of::<usize>(),
    rust_trap_exception = sym rust_trap_exception,
    options(noreturn))
}

#[repr(C)]
struct TrapFrame {
    ra: usize,
    t0: usize,
    t1: usize,
    t2: usize,
    t3: usize,
    t4: usize,
    t5: usize,
    t6: usize,
    a0: usize,
    a1: usize,
    a2: usize,
    a3: usize,
    a4: usize,
    a5: usize,
    a6: usize,
    a7: usize,
    tp: usize,
}

/// 根据硬件线程号设置启动栈。
///
/// # Safety
///
/// 裸函数。
#[naked]
unsafe extern "C" fn select_stack(hartid: usize) {
    #[link_section = ".bss.uninit"]
    static mut BOOT_STACK: [u8; STACK_SIZE] = [0u8; STACK_SIZE];

    asm!("
           mv   tp, a0
           addi t0, a0,  1
           la   sp, {stack}
           li   t1, {len_per_hart}
        1: add  sp, sp, t1
           addi t0, t0, -1
           bnez t0, 1b
           ret
        ",
        stack = sym BOOT_STACK,
        len_per_hart = const PER_HART_STACK_SIZE,
        options(noreturn)
    )
}

/// 清零 bss 段。
#[inline(always)]
fn zero_bss() {
    #[cfg(target_pointer_width = "32")]
    type Word = u32;
    #[cfg(target_pointer_width = "64")]
    type Word = u64;
    extern "C" {
        static mut sbss: Word;
        static mut ebss: Word;
    }
    unsafe { r0::zero_bss(&mut sbss, &mut ebss) };
}

struct BoardInfo {
    smp: usize,
    uart: usize,
}

fn parse_smp(dtb_pa: usize) -> BoardInfo {
    use dtb_walker::{Dtb, DtbObj, HeaderError as E, Property, Str, WalkOperation::*};

    let mut ans = BoardInfo { smp: 0, uart: 0 };
    unsafe {
        Dtb::from_raw_parts_filtered(dtb_pa as _, |e| {
            matches!(e, E::Misaligned(4) | E::LastCompVersion(16))
        })
    }
    .unwrap()
    .walk(|ctx, obj| match obj {
        DtbObj::SubNode { name } => {
            if ctx.is_root() && (name == Str::from("cpus") || name == Str::from("soc")) {
                StepInto
            } else if ctx.name() == Str::from("cpus") && name.starts_with("cpu@") {
                ans.smp += 1;
                StepOver
            } else if ctx.name() == Str::from("soc") && name.starts_with("uart") {
                StepInto
            } else {
                StepOver
            }
        }
        DtbObj::Property(Property::Reg(mut reg)) => {
            if ctx.name().starts_with("uart") {
                ans.uart = reg.next().unwrap().start;
            }
            StepOut
        }
        DtbObj::Property(_) => StepOver,
    });
    ans
}
