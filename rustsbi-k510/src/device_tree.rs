use core::{
    fmt::{Display, Formatter, Result},
    ops::Range,
};

/// 从设备树采集的板信息。
pub(crate) struct BoardInfo {
    pub dtb: Range<usize>,
    pub model: StringInline<128>,
    pub smp: usize,
    pub mem: Range<usize>,
    pub serial_count: usize,
    pub serial: [Range<usize>; 3], // k510有多个uart输出，引导启动时使用0号uart
    pub plic_count: usize,
    pub plic: [Range<usize>; 2],   // plic, k510 使用一个特殊的类plic组件来控制ipi, 也就是名为plic_sw的plic[1]
    pub plmt: Range<usize>,        // andes Platform-Level Machine Timer
}

/// 在栈上存储有限长度字符串。
pub(crate) struct StringInline<const N: usize>(usize, [u8; N]);

impl<const N: usize> Display for StringInline<N> {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "{}", unsafe {
            core::str::from_utf8_unchecked(&self.1[..self.0])
        })
    }
}

/// 解析设备树。
pub(crate) fn parse(opaque: usize) -> BoardInfo {
    use dtb_walker::{Dtb, DtbObj, HeaderError as E, Property, Str, WalkOperation::*};
    const CPUS: &str = "cpus";
    const MEMORY: &str = "memory";
    const SERIAL: &str = "serial";
    const PLIC: &str = "interrupt-controller";
    const PLMT: &str = "plmt";
    const SOC: &str = "soc";

    let mut ans = BoardInfo {
        dtb: opaque..opaque,
        model: StringInline(0, [0u8; 128]),
        smp: 0,
        mem: 0..0,
        serial_count: 0,
        serial: [0..0, 0..0, 0..0],
        plic_count: 0,
        plic: [0..0, 0..0],
        plmt: 0..0,
    };
    let dtb = unsafe {
        Dtb::from_raw_parts_filtered(opaque as _, |e| {
            matches!(e, E::Misaligned(4) | E::LastCompVersion(16))
        })
    }
    .unwrap();
    ans.dtb.end += dtb.total_size();
    dtb.walk(|ctx, obj| match obj {
        DtbObj::SubNode { name } => {
            let current = ctx.name();
            if ctx.is_root() {
                if name == Str::from(CPUS)
                    || name == Str::from(SOC)
                    || name.starts_with(MEMORY)
                    || name.starts_with(SERIAL) {
                    StepInto
                } else {
                    StepOver
                }
            } else if current == Str::from(SOC) {
                if name.starts_with(PLIC) || name.starts_with(PLMT){
                    StepInto
                } else {
                    StepOver
                }
            } else {
                if current == Str::from(CPUS) && name.starts_with("cpu@") {
                    ans.smp += 1;
                }
                StepOver
            }
        }
        DtbObj::Property(Property::Model(model)) if ctx.is_root() => {
            ans.model.0 = model.as_bytes().len();
            ans.model.1[..ans.model.0].copy_from_slice(model.as_bytes());
            StepOver
        }
        DtbObj::Property(Property::Reg(mut reg)) => {
            let node = ctx.name();
            if node.starts_with(SERIAL) {
                ans.serial[ans.serial_count] = reg.next().unwrap();
                ans.serial_count += 1;
                StepOut
            }  else if node.starts_with(PLIC) {
                ans.plic[ans.plic_count] = reg.next().unwrap();
                ans.plic_count += 1;
                StepOut
            } else if node.starts_with(MEMORY) {
                ans.mem = reg.next().unwrap();
                StepOut
            } else if node.starts_with(PLMT) {
                ans.plmt = reg.next().unwrap();
                StepOut
            }
            else {
                StepOver
            }
        }
        DtbObj::Property(_) => StepOver,
    });

    ans
}
