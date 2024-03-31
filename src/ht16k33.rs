use esp_hal::prelude::*;
use esp_hal::{i2c::I2C, peripherals::I2C0, Delay};
use esp_println::println;
pub struct HT16K33 {
    i2c: I2C<'static, I2C0>,
    addr: u8,
    display_state: DisplayState,
    pub display_buffer : [u8;15]
}

#[derive(Copy, Clone)]
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
        HT16K33 {
            i2c,
            addr,
            display_state: DisplayState::OFF,
            display_buffer:[0;15],
        }
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

        // DisplayOn and blink off
        let display_setup = Self::DISPLAY_SETUP | 0x01;
        self.i2c.write(self.addr, &[display_setup]).ok();
        self.display_state = DisplayState::ON;

        // Waiting for the setup to be completed
        delay.delay_ms(2u8);

        self.clear();
    }

    /// set the state of the display
    /// This set the blink state to off
    pub fn set_display_state(&mut self, state: DisplayState) {
        self.i2c
            .write(self.addr, &[Self::DISPLAY_SETUP | state as u8])
            .ok();
        self.display_state = state;
    }

    /// set the brightness of the display
    pub fn set_brightness(&mut self, brightness: u32) {
        // have the brigthness such as 100 ---> 0xff and 0 ---> 0
        let mut in_range_brightness = brightness;

        // put the brigthness in the range 0 ---> 100
        if in_range_brightness > 100 {
            in_range_brightness = 100;
        }

        // have the brigthness such as 100 ---> 0xff and 0 ---> 0
        in_range_brightness = (in_range_brightness) * 15 / 100;
        in_range_brightness = in_range_brightness % 16;
        println!("brigthness is {in_range_brightness}");

        // writing the value to the ht16K33 register
        self.i2c
            .write(self.addr, &[Self::DIMMING_SET | in_range_brightness as u8])
            .ok();
    }

    /// set the blink rate of the display
    pub fn set_blink_rate(&mut self, rate: DisplayBlinkRate) {
        self.i2c
            .write(
                self.addr,
                &[Self::DISPLAY_SETUP | ((rate as u8) << 1) | self.display_state as u8],
            )
            .ok();
    }

    /// Issue buffered data in RAM to display
    pub fn write_to_display(&mut self) {
        let mut data_and_cmd = [0u8;16];
        // the first value we must send is the display data pointer address
        data_and_cmd[0] = Self::DISPLAY_DATA_POINTER;

        // copy the ram data to the array we are sending
        for j in 1..=15{
            data_and_cmd[j] = self.display_buffer[j-1];
        }
        // write the data to the display ram
        self.i2c.write(self.addr, &data_and_cmd).ok();
    }

    /// clear the display
    /// it assumes that clearing the display means setting all rows bits to 0
    pub fn clear(&mut self) {
       self.display_buffer = [0;15];
       self.write_to_display();
    }
}
