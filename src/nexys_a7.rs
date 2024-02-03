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

    pub fn cpu_resetn() -> Signal<In, Bit> {
        let mut x = Signal::<In, _>::default();
        x.add_location(0, "C12");
        x.add_signal_type(0, SignalType::LowVoltageCMOS_3v3);
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
    pub fn rgb_led16() -> Signal<Out, Bits<3>> {
        let mut x = Signal::<Out, _>::default();
        for (ndx, uname) in ["N15", "M16", "R12"].iter().enumerate() {
            x.add_location(ndx, uname);
            x.add_signal_type(ndx, SignalType::LowVoltageCMOS_3v3);
        }
        x
    }
    pub fn rgb_led17() -> Signal<Out, Bits<3>> {
        let mut x = Signal::<Out, _>::default();
        for (ndx, uname) in ["G14", "R11", "N16"].iter().enumerate() {
            x.add_location(ndx, uname);
            x.add_signal_type(ndx, SignalType::LowVoltageCMOS_3v3);
        }
        x
    }

    pub fn seven_seg_cath() -> Signal<Out, Bits<8>> {
        let mut x = Signal::<Out, _>::default();
        for (ndx, uname) in ["T10", "R10", "K16", "K13", "P15", "T11", "L18", "H15"]
            .iter()
            .enumerate()
        {
            x.add_location(ndx, uname);
            x.add_signal_type(ndx, SignalType::LowVoltageCMOS_3v3);
        }
        x
    }
    pub fn seven_seg_anodes() -> Signal<Out, Bits<8>> {
        let mut x = Signal::<Out, _>::default();
        for (ndx, uname) in ["J17", "J18", "T9", "J14", "P14", "T14", "K2", "U13"]
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
