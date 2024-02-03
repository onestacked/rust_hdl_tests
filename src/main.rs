use blinky::Blinky;
use nexys_a7::CLOCK_SPEED_100MHZ;
use rust_hdl::{fpga::toolchains::vivado::generate_xdc, prelude::*};
use std::{fs::File, io::Write};
mod nexys_a7;

use clap::{Parser, Subcommand};
mod blinky;
mod counter;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
#[command(propagate_version = true)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Build,
    Sim,
}

fn main() {
    let cli = Cli::parse();

    // You can check for the existence of subcommands, and if found use their
    // matches just as you would the top level cmd
    match cli.command {
        Commands::Build => build(),
        Commands::Sim => sim(),
    }
}

fn build() {
    let mut uut = Blinky::default();
    //synth::generate_bitstream(uut, "firmware/blinky")
    uut.connect_all();

    let mut xdc = File::create("build/top.xdc").unwrap();
    xdc.write_all(generate_xdc(&uut).as_bytes()).unwrap();

    let mut verilog = File::create("build/top.v").unwrap();
    verilog
        .write_all(generate_verilog(&uut).as_bytes())
        .unwrap();
}

fn sim() {
    // v--- build a simple simulation (1 testbench, single clock)
    let mut sim = simple_sim!(Blinky, clock, CLOCK_SPEED_100MHZ, ep, {
        let mut x = ep.init()?;
        wait_clock_cycles!(ep, clock, x, 10);
        x.resetn.next = true;
        wait_clock_cycles!(ep, clock, x, CLOCK_SPEED_100MHZ / 1000);
        x.sw.next = 0xABCD.into();
        wait_clock_cycles!(ep, clock, x, CLOCK_SPEED_100MHZ / 1000);
        ep.done(x)
    });

    // v--- construct the circuit
    let uut = Blinky::default();
    // v--- run the simulation, with the output traced to a .vcd file
    sim.run_to_file(Box::new(uut), 5 * sim_time::ONE_SEC, "blinky.vcd")
        .unwrap();
}
