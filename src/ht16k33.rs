use esp_hal::{i2c::I2C, peripherals::I2C0};

pub struct HT16K33 {
    i2c: I2C<'static, I2C0>,
    addr: usize,
    display_buffer: [u16; 8],
}

pub enum DisplayState {
    ON,
    OFF,
}

pub enum DisplayBlinkRate {
    OFF,
    Blink2Hz,
    Blink1Hz,
    BlinkHalfHz,
}

impl HT16K33 {
    /// returns a new ht16k33 device
    pub fn new(i2c: I2C<'static, I2C0>, addr: usize) -> Self {
        todo!()
    }

    /// set the state of the display
    pub fn set_display_state(state: DisplayState) {
        todo!()
    }

    /// set the brightness of the display
    pub fn set_brightness(brigthness: u8) {
        todo!()
    }

    /// set the blink rate of the display
    pub fn set_blink_rate(rate: DisplayBlinkRate) {
        
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