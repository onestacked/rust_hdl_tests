use rust_hdl::prelude::*;

use crate::{counter::Counter, nexys_a7::pins};

#[derive(LogicBlock)]
pub struct Blinky {
    pub clock: Signal<In, Clock>,
    pub resetn: Signal<In, Bit>,
    pub sw: Signal<In, Bits<16>>,
    pub leds: Signal<Out, Bits<16>>,
    counter: Counter<36>,
}
impl Logic for Blinky {
    #[hdl_gen]
    fn update(&mut self) {
        self.counter.clock.next = self.clock.val();
        self.counter.reset.next = !self.resetn.val();

        self.leds.next = self.sw.val() ^ self.counter.out.val().get_bits::<16>(20);
    }
}

impl Default for Blinky {
    fn default() -> Self {
        Blinky {
            counter: Default::default(),
            resetn: pins::cpu_resetn(),
            clock: pins::clock(),
            leds: pins::leds(),
            sw: pins::switches(),
        }
    }
}
