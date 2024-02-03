pub const CLOCK_SPEED_100MHZ: u64 = 100_000_000;
#[allow(unused)]
pub mod pins {
    use rust_hdl::prelude::*;

    pub fn clock() -> Signal<In, Clock> {
        let mut x = Signal::<In, _>::default();
        x.add_location(0, "E3");
        x.add_signal_type(0, SignalType::LowVoltageCMOS_3v3);
        x.add_constraint(PinConstraint {
            index: 0,
            constraint: Constraint::Timing(Periodic(PeriodicTiming {
                net: "sys_clk_pin".into(),
                period_nanoseconds: 10.0,
                duty_cycle: 50.0,
            })),
        });
        x.connect();
        x
    }

    pub fn leds() -> Signal<Out, Bits<16>> {
        let mut x = Signal::<Out, _>::default();
        for (ndx, uname) in [
            "H17", "K15", "J13", "N14", "R18", "V17", "U17", "U16", "V16", "T15", "U14", "T16",
            "V15", "V14", "V12", "V11",
        ]
        .iter()
        .enumerate()
        {
            x.add_location(ndx, uname);
            x.add_signal_type(ndx, SignalType::LowVoltageCMOS_3v3);
        }
        x
    }
    pub fn switches() -> Signal<In, Bits<16>> {
        let mut x = Signal::<In, _>::default();
        for (ndx, uname) in [
            "J15", "L16", "M13", "R15", "R17", "T18", "U18", "R13", "T8", "U8", "R16", "T13", "H6",
            "U12", "U11", "V10",
        ]
        .iter()
        .enumerate()
        {
            x.add_location(ndx, uname);
            if ndx == 8 || ndx == 9 {
                x.add_signal_type(ndx, SignalType::LowVoltageCMOS_1v8);
            } else {
                x.add_signal_type(ndx, SignalType::LowVoltageCMOS_3v3);
            }
        }
        x
    }
    pub fn btnc() -> Signal<In, Bit> {
        let mut x = Signal::<In, _>::default();
        x.add_location(0, "N17");
        x
    }
    pub fn btnu() -> Signal<In, Bit> {
        let mut x = Signal::<In, _>::default();
        x.add_location(0, "M18");
        x
    }
    pub fn btnl() -> Signal<In, Bit> {
        let mut x = Signal::<In, _>::default();
        x.add_location(0, "P17");
        x
    }
    pub fn btnr() -> Signal<In, Bit> {
        let mut x = Signal::<In, _>::default();
        x.add_location(0, "M17");
        x
    }
    pub fn btnd() -> Signal<In, Bit> {
        let mut x = Signal::<In, _>::default();
        x.add_location(0, "M17");
        x
    }
}
