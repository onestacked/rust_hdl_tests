use nexys_a7::{pins, CLOCK_SPEED_100MHZ};
use rust_hdl::prelude::*;
use std::{fs::File, io::Write, time::Duration};
mod nexys_a7;

#[derive(LogicBlock)]
pub struct Blinky {
    pulser: Pulser,
    clock: Signal<In, Clock>,
    leds: Signal<Out, Bits<16>>,
}
impl Logic for Blinky {
    #[hdl_gen]
    fn update(&mut self) {
        self.pulser.enable.next = true;
        self.pulser.clock.next = self.clock.val();
        self.leds.next = 0x00.into();
        if self.pulser.pulse.val() {
            self.leds.next = 0xAAAA.into();
        }
    }
}

impl Default for Blinky {
    fn default() -> Self {
        let pulser = Pulser::new(CLOCK_SPEED_100MHZ.into(), 1.0, Duration::from_millis(250));
        Blinky {
            pulser,
            clock: pins::clock(),
            leds: pins::leds(),
        }
    }
}

fn main() {
    //sim();
    build();
}

#[allow(unused)]
fn build() {
    let mut uut = Blinky::default();
    //synth::generate_bitstream(uut, "firmware/blinky")
    uut.connect_all();
    let vlog = generate_verilog(&uut);
    let mut out = File::create("build/top.v").unwrap();
    out.write_all(vlog.as_bytes());

    // TODO: Build xdc
    //println!("{vlog}")
}

#[allow(unused)]
fn sim() {
    // v--- build a simple simulation (1 testbench, single clock)
    let mut sim = simple_sim!(Blinky, clock, CLOCK_SPEED_100MHZ, ep, {
        let mut x = ep.init()?;
        wait_clock_cycles!(ep, clock, x, 4 * CLOCK_SPEED_100MHZ);
        ep.done(x)
    });

    // v--- construct the circuit
    let uut = Blinky::default();
    // v--- run the simulation, with the output traced to a .vcd file
    sim.run_to_file(Box::new(uut), 5 * sim_time::ONE_SEC, "blinky.vcd");
}
