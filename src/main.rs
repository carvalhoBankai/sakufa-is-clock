#![no_std]
#![no_main]

use core::fmt::Display;

use esp_backtrace as _;
use esp_hal::{
    clock::ClockControl, gpio::IO, i2c::I2C, peripherals::Peripherals, prelude::*, Delay,
};
use esp_println::println;
mod ht16k33;
mod seven_seg_display;
use crate::ht16k33::*;
use crate::seven_seg_display::*;

#[entry]
fn main() -> ! {
    let peripherals = Peripherals::take();
    let system = peripherals.SYSTEM.split();

    let clocks = ClockControl::max(system.clock_control).freeze();
    let mut delay = Delay::new(&clocks);
    let io = IO::new(peripherals.GPIO, peripherals.IO_MUX);

    let i2c = I2C::new(
        peripherals.I2C0,
        io.pins.gpio21,
        io.pins.gpio22,
        100u32.kHz(),
        &clocks,
    );

    //let mut display = HT16K33::new(i2c, 0x70);

    //display.init(&mut delay);

    //display.set_blink_rate(DisplayBlinkRate::Blink2Hz);

    // display.display_buffer[0] = 0xff;
    // display.display_buffer[2] = 0xf0;

    // display.write_to_display();
    // delay.delay_ms(500u32);

    let mut seven_seg_display = SEVENSEGDISPLAY::new(4, i2c, 0x70, &mut delay);

    

    
    // setup logger
    // To change the log_level change the env section in .cargo/config.toml
    // or remove it and set ESP_LOGLEVEL manually before running cargo run
    // this requires a clean rebuild because of https://github.com/rust-lang/cargo/issues/10358
    esp_println::logger::init_logger_from_env();
    log::info!("Logger is setup");
    //seven_seg_display.display_number(423);
    seven_seg_display.display_string("3D3D");
    
    loop {
       // println!("Loop...");
        /* for i in 0..4{
            for j in 0..10{
                seven_seg_display.print_number(j,i);
                delay.delay_ms(200u32);
            }
        } */
    }
}
