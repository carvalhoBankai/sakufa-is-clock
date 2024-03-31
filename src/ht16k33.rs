use esp_hal::prelude::*;
use esp_hal::{i2c::I2C, peripherals::I2C0, Delay};
use esp_println::println;
pub struct HT16K33 {
    i2c: I2C<'static, I2C0>,
    addr: u8,
}

pub enum DisplayState {
    OFF,
    ON,
}

pub enum DisplayBlinkRate {
    OFF,
    Blink2Hz,
    Blink1Hz,
    BlinkHalfHz,
}

impl HT16K33 {
    const DISPLAY_DATA_POINTER: u8 = 0;
    const SYTEM_SETUP: u8 = 0x20;
    const DISPLAY_SETUP: u8 = 0x80;
    const ROW_INT: u8 = 0xa0;
    const DIMMING_SET: u8 = 0xe0;

    /// returns a new ht16k33 device
    pub fn new(i2c: I2C<'static, I2C0>, addr: u8) -> Self {
        HT16K33 { i2c, addr }
    }

    /// Initialise the ht16k33 module
    /// It is recommended to wait 1ms before calling this method after a power reset
    pub fn init(&mut self, delay: &mut Delay) {
        // Enable the system clock
        let osc_setup = Self::SYTEM_SETUP | 0x01;
        self.i2c.write(self.addr, &[osc_setup]).ok();

        // INT/ROW as output
        let row_int_setup = Self::ROW_INT;
        self.i2c.write(self.addr, &[row_int_setup]).ok();

        // DisplayOn and blink at 2Hz
        let display_setup = Self::DISPLAY_SETUP | 0x01;
        self.i2c.write(self.addr, &[display_setup]).ok();

        // Waiting for the setup to be completed
        delay.delay_ms(2u8);
    }

    /// set the state of the display
    /// This set the blink state to off
    pub fn set_display_state(&mut self, state: DisplayState) {
        self.i2c
            .write(self.addr, &[Self::DISPLAY_SETUP | state as u8])
            .ok();
    }

    /// set the brightness of the display
    pub fn set_brightness(&mut self, brigthness: u32) {
       todo!()
    }

    /// set the blink rate of the display
    pub fn set_blink_rate(rate: DisplayBlinkRate) {
        todo!()
    }

    /// Issue buffered data in RAM to display
    pub fn write_to_display(buffer_to_write: &[u16]) {
        todo!()
    }

    /// clear the display
    pub fn clear() {
        todo!()
    }
}
