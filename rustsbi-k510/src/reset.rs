pub struct Reset;

impl rustsbi::Reset for Reset {
    fn system_reset(&self, reset_type: u32, reset_reason: u32) -> rustsbi::spec::binary::SbiRet {
        println!("[rustsbi] reset triggered! todo: shutdown all harts on K510; program halt. Type: {}, reason: {}", reset_type, reset_reason);
        loop {}
    }
}