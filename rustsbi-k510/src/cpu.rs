#[inline(always)]
pub(crate) fn hart_id() -> usize {
    riscv::register::mhartid::read()
}