use crate::{hart_id, k510_hsm::K510Hsm, Supervisor, plmt, plic_sw};
use core::arch::asm;
use riscv::register::*;

#[repr(usize)]
pub(crate) enum Operation {
    Stop = 0,
    SystemReset = 1,
}

pub(crate) fn execute_supervisor(hsm: &K510Hsm, supervisor: Supervisor) -> Operation {
    let mut ctx = SupervisorContext::new(supervisor);
    mscratch::write(&mut ctx as *mut _ as _);

    plic_sw::clear_ipi();
    plmt::mtimecmp::clear();
    unsafe {
        asm!("csrw mideleg, {}", in(reg) !0);
        asm!("csrw medeleg, {}", in(reg) !0);
        asm!("csrw mcounteren, {}", in(reg) !0);
        medeleg::clear_supervisor_env_call();
        medeleg::clear_machine_env_call();

        mtvec::write(trap_vec as _, mtvec::TrapMode::Vectored);
        mie::set_mext();
        mie::set_msoft();
        mie::set_mtimer();
    }

    hsm.record_current_start_finished();
    loop {
        unsafe { m_to_s() };

        use mcause::{Exception, Trap};
        match mcause::read().cause() {
            Trap::Exception(Exception::SupervisorEnvCall) => {
                if let Some(op) = ctx.handle_ecall() {
                    break op;
                }
            }
            t => ctx.trap_stop(t),
        }
    }
}

#[repr(C)]
#[derive(Debug)]
struct SupervisorContext {
    msp: usize,
    x: [usize; 31],
    mstatus: usize,
    mepc: usize,
}

impl SupervisorContext {
    fn new(supervisor: Supervisor) -> Self {
        let mut ctx = Self {
            msp: 0,
            x: [0; 31],
            mstatus: 0,
            mepc: supervisor.start_addr,
        };

        unsafe {
            mstatus::set_mpp(mstatus::MPP::Supervisor);
            mstatus::set_mpie();
            asm!("csrr {}, mstatus", out(reg) ctx.mstatus)
        };
        *ctx.a_mut(0) = hart_id();
        *ctx.a_mut(1) = supervisor.opaque;

        ctx
    }

    #[inline]
    fn x(&self, n: usize) -> usize {
        self.x[n - 1]
    }

    #[inline]
    fn x_mut(&mut self, n: usize) -> &mut usize {
        &mut self.x[n - 1]
    }

    #[inline]
    fn a(&self, n: usize) -> usize {
        self.x(n + 10)
    }

    #[inline]
    fn a_mut(&mut self, n: usize) -> &mut usize {
        self.x_mut(n + 10)
    }

    fn handle_ecall(&mut self) -> Option<Operation> {
        use rustsbi::spec::{binary::*, hsm::*, srst::*};
        let extension = self.a(7);
        let function = self.a(6);
        let ans = rustsbi::ecall(
            extension,
            function,
            [
                self.a(0),
                self.a(1),
                self.a(2),
                self.a(3),
                self.a(4),
                self.a(5),
            ],
        );
        // ???????????????????????????????????????
        if ans.error == RET_SUCCESS {
            match extension {
                // ?????????
                EID_HSM => match function {
                    HART_STOP => return Some(Operation::Stop),
                    HART_SUSPEND
                    if matches!(
                            u32::try_from(self.a(0)),
                            Ok(HART_SUSPEND_TYPE_NON_RETENTIVE)
                        ) =>
                        {
                            return Some(Operation::Stop);
                        }
                    _ => {}
                },
                // ????????????
                EID_SRST => match function {
                    SYSTEM_RESET
                    if matches!(
                            u32::try_from(self.a(0)),
                            Ok(RESET_TYPE_COLD_REBOOT) | Ok(RESET_TYPE_WARM_REBOOT)
                        ) =>
                        {
                            return Some(Operation::SystemReset)
                        }
                    _ => {}
                },

                _ => {}
            }
        }
        *self.a_mut(0) = ans.error;
        *self.a_mut(1) = ans.value;
        self.mepc = self.mepc.wrapping_add(4);
        None
    }

    #[allow(unused)]
    fn do_transfer_trap(&mut self, cause: scause::Trap) {
        unsafe {
            // ??? S ????????????
            mstatus::set_mpp(mstatus::MPP::Supervisor);
            // ?????????????????????
            let spp = match (self.mstatus >> 11) & 0b11 {
                // U
                0b00 => mstatus::SPP::User,
                // S
                0b01 => mstatus::SPP::Supervisor,
                // H/M
                mpp => unreachable!("invalid mpp: {mpp:#x} to delegate"),
            };
            mstatus::set_spp(spp);
            // ??????????????????
            scause::set(cause);
            // ????????????????????????
            stval::write(mtval::read());
            // ??????????????????
            sepc::write(self.mepc);
            // ?????? S ????????????
            if mstatus::read().sie() {
                mstatus::set_spie();
                mstatus::clear_sie();
            }
            asm!("csrr {}, mstatus", out(reg) self.mstatus);
            // ?????????????????????????????? S
            // TODO Vectored stvec?
            self.mepc = stvec::read().address();
        }
    }

    fn trap_stop(&self, trap: mcause::Trap) -> ! {
        println!(
            "
-----------------------------
> trap:    {trap:?}
> mstatus: {:#018x}
> mepc:    {:#018x}
> mtval:   {:#018x}
-----------------------------
",
            self.mstatus,
            self.mepc,
            mtval::read()
        );
        panic!("stopped with unsupported trap")
    }
}

/// M ????????? S ??????
///
/// # Safety
///
/// ????????????????????????????????????????????????
/// ??????????????????????????? 32 * usize ??????????????? 31 ????????? 31 ?????????????????????
/// ?????? x0(zero) ??? x2(sp) ???????????????????????????
#[naked]
unsafe extern "C" fn m_to_s() {
    asm!(
        r"  .altmacro
            .macro SAVE_M n
                sd x\n, \n*8(sp)
            .endm
            .macro LOAD_S n
                ld x\n, \n*8(sp)
            .endm
        ",
        // ??????????????????sp = Mctx
        "   addi sp, sp, -32*8",
        // ?????????????????????????????????????????????
        "   csrr  t0, mscratch
            sd    t0, (sp)
        ",
        // ?????????????????????
        "   .set n, 1
            .rept 31
                SAVE_M %n
                .set n, n+1
            .endr
        ",
        // ??????????????????sp = Sctx
        "   csrrw sp, mscratch, sp",
        // ?????????????????????????????????????????????
        "   csrr  t0, mscratch
            sd    t0, (sp)
        ",
        // ?????? csr
        "   ld   t0, 32*8(sp)
            ld   t1, 33*8(sp)
            csrw mstatus, t0
            csrw    mepc, t1
        ",
        // ?????????????????????
        "   ld x1, 1*8(sp)
            .set n, 3
            .rept 29
                LOAD_S %n
                .set n, n+1
            .endr
            ld sp, 2*8(sp)
        ",
        // ??????????????????
        "   mret",
        options(noreturn)
    )
}

/// S ????????? M ??????
///
/// # Safety
///
/// ????????????
/// ??????????????? ra ?????? [`m_to_s`] ??????????????????
#[naked]
unsafe extern "C" fn s_to_m() {
    asm!(
        r"
        .altmacro
        .macro SAVE_S n
            sd x\n, \n*8(sp)
        .endm
        .macro LOAD_M n
            ld x\n, \n*8(sp)
        .endm
        ",
        // ??????????????????sp = Sctx
        "   csrrw sp, mscratch, sp
            ld    sp, (sp)
        ",
        // ?????????????????????
        "   sd x1, 1*8(sp)
            .set n, 3
            .rept 29
                SAVE_S %n
                .set n, n+1
            .endr
            csrrw t0, mscratch, sp
            sd    t0,  2*8(sp)
        ",
        // ?????? csr
        "   csrr t1, mstatus
            csrr t2, mepc
            sd   t1, 32*8(sp)
            sd   t2, 33*8(sp)
        ",
        // ??????????????????sp = Mctx
        "   ld sp, (sp)",
        // ?????????????????????
        "   .set n, 1
            .rept 31
                LOAD_M %n
                .set n, n+1
            .endr
        ",
        // ?????????????????????
        "   addi sp, sp, 32*8
            ret
        ",
        options(noreturn)
    )
}

/// ???????????????
///
/// # Safety
///
/// ????????????
#[naked]
unsafe extern "C" fn trap_vec() {
    asm!(
    ".align 2",
    ".option push",
    ".option norvc",
    "j {s_to_m}", // exception
    "j {s_to_m}", // supervisor software
    "j {s_to_m}", // reserved
    "j {msoft} ", // machine    software
    "j {s_to_m}", // reserved
    "j {s_to_m}", // supervisor timer
    "j {s_to_m}", // reserved
    "j {mtimer}", // machine    timer
    "j {s_to_m}", // reserved
    "j {s_to_m}", // supervisor external
    "j {s_to_m}", // reserved
    "j {s_to_m}", // machine    external
    ".option pop",
    s_to_m = sym s_to_m,
    mtimer = sym mtimer,
    msoft  = sym msoft,
    options(noreturn)
    )
}

/// machine timer ????????????
///
/// # Safety
///
/// ????????????
#[naked]
unsafe extern "C" fn mtimer() {
    asm!(
    // ?????????
    // sp      : M sp
    // mscratch: S sp
    "   csrrw sp, mscratch, sp",
    // ?????? a0 ???????????????
    "   addi sp, sp, -16
            sd   ra, 0(sp)
            sd   a0, 8(sp)
        ",
    // clint::mtimecmp::clear();
    "   li   a0, {u64_max}
            call {set_mtimecmp}
        ",
    // mip::set_stimer();
    "   li   a0, {mip_stip}
           csrrs zero, mip, a0
        ",
    // ?????? a0
    "   ld   a0, 8(sp)
            ld   ra, 0(sp)
            addi sp, sp,  16
        ",
    // ?????????
    // sp      : S sp
    // mscratch: M sp
    "   csrrw sp, mscratch, sp",
    // ??????
    "   mret",
    u64_max      = const u64::MAX,
    mip_stip     = const 1 << 5,
    set_mtimecmp =   sym plmt::mtimecmp::set_naked,
    options(noreturn)
    )
}

/// machine soft ????????????
///
/// # Safety
///
/// ????????????
#[naked]
unsafe extern "C" fn msoft() {
    asm!(
    // ?????????
    // sp      : M sp
    // mscratch: S sp
    "   csrrw sp, mscratch, sp",
    // ?????? ra
    "   addi sp, sp, -8
            sd   ra, 0(sp)
        ",
    // clint::msip::clear();
    // mip::set_ssoft();
    "   call   {clear_msip}
            csrrsi zero, mip, 1 << 1
        ",
    // ?????? ra
    "   ld   ra, 0(sp)
            addi sp, sp,  8
        ",
    // ?????????
    // sp      : S sp
    // mscratch: M sp
    "   csrrw sp, mscratch, sp",
    // ??????
    "   mret",
    clear_msip = sym plic_sw::clear_ipi,
    options(noreturn)
    )
}