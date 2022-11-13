


struct Ws2812Data {
    blue: u32,
    green: u32,
    red: u32,
    reserved: u32
}

pub(crate) struct Ws2812Info {
    ws_nun: usize,
    ws_data: Ws2812Data,
}

mod constants {
    pub(crate) const WS2812_PIN: u32 = 122;
}

use constants::*;