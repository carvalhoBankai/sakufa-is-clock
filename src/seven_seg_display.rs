use crate::ht16k33::{self, *};
use esp_hal::prelude::*;
use esp_hal::{i2c::I2C, peripherals::I2C0, Delay};
pub struct SEVENSEGDISPLAY {
    cursor: u8,
    ht16k33: HT16K33,
    digits: u8,
    display_map: [usize; 16],
}

impl SEVENSEGDISPLAY {
    const DISPLAYNUMBERS: [u8; 11] = [
        0xbf, 0x86, 0xdb, 0xcf, 0xe6, 0xed, 0xfd, 0x87, 0xff, 0xe7, 0xdf,
    ];

    pub fn new(digits: u8, i2c: I2C<'static, I2C0>, addr: u8, delay: &mut Delay) -> Self {
        let mut ht16k33 = HT16K33::new(i2c, addr);
        ht16k33.init(delay);
        ht16k33.clear();
        
        let mut seven_seg_display = 
        SEVENSEGDISPLAY {
            cursor: 0,
            ht16k33,
            digits,
            display_map: [0, 2, 6, 8, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
        };

        // initialise all digits with 0
        for dig in 0..digits {
            seven_seg_display.ht16k33.display_buffer[seven_seg_display.display_map[dig as usize]] =
            Self::DISPLAYNUMBERS[0];
        }
        seven_seg_display
    }

    /// prints a number between 0 and 9 at position position of the display
    pub fn print_number(&mut self, number: u8, position: usize) {
        self.ht16k33.display_buffer[self.display_map[position]] =
            Self::DISPLAYNUMBERS[number as usize];
        self.ht16k33.write_to_display();
    }
}
