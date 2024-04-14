use crate::ht16k33::{self, *};
use esp_hal::{i2c::I2C, peripherals::I2C0, Delay};
use esp_println::println;
use heapless::*;
pub struct SEVENSEGDISPLAY {
    cursor: u8,
    ht16k33: HT16K33,
    number_of_digits: u8,
    display_map: [usize; 16],
}

impl SEVENSEGDISPLAY {
    const DISPLAYNUMBERS: [u8; 11] = [
        0xbf, 0x86, 0xdb, 0xcf, 0xe6, 0xed, 0xfd, 0x87, 0xff, 0xe7, 0xdf,
    ];

    const DISPLAY_LETTERS: [u8; 24] = [
        0x77, 0x7c, 0x39, 0x5e, 0x79, 0x71, 0x7c, 0x74, 0x30, 0x1e, 0x76, 0x38, 0x37, 0x37, 0x3f,
        0x73, 0x3f, 0x77, 0x6d, 0x07, 0x3e, 0x3e, 0x76, 0x5b,
    ];
    pub fn new(number_of_digits: u8, i2c: I2C<'static, I2C0>, addr: u8, delay: &mut Delay) -> Self {
        let mut ht16k33 = HT16K33::new(i2c, addr);
        ht16k33.init(delay);
        ht16k33.clear();

        let mut seven_seg_display = SEVENSEGDISPLAY {
            cursor: 0,
            ht16k33,
            number_of_digits,
            display_map: [0, 2, 6, 8, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
        };

        // initialise all digits with 0
        for dig in 0..number_of_digits {
            seven_seg_display.ht16k33.display_buffer[seven_seg_display.display_map[dig as usize]] =
                Self::DISPLAYNUMBERS[0];
        }
        seven_seg_display.ht16k33.write_to_display();
        seven_seg_display
    }

    /// prints a number between 0 and 9 at position position of the display
    pub fn print_number(&mut self, number: u8, position: usize) {
        self.ht16k33.display_buffer
            [self.display_map[position % (self.number_of_digits as usize)]] =
            Self::DISPLAYNUMBERS[(number % Self::DISPLAYNUMBERS.len() as u8) as usize];
        self.ht16k33.write_to_display();
    }

    /// display a number that fits on the display
    /// if the number does not fit in the display, it will only display
    /// number of digits last numbers
    pub fn display_number(&mut self, number: u32) {
        let mut num: u32 = number;
        for pos in 0..self.number_of_digits {
            self.print_number((num % 10) as u8, (self.number_of_digits - pos - 1) as usize);
            num = num / 10;
        }
    }

    ///display a single caracter on a position pos
    /// if it is not possible to display it, nothing will hapen
    pub fn display_char(&mut self, car: char, position: usize) {
        let car = car.to_ascii_uppercase() as u8;
        if car < 65 || car > 90 {
            return;
        }

        let number = car - 65;
        self.ht16k33.display_buffer
            [self.display_map[position % (self.number_of_digits as usize)]] =
            Self::DISPLAY_LETTERS[(number % Self::DISPLAY_LETTERS.len() as u8) as usize];
        self.ht16k33.write_to_display();
    }

    /// display a string that fits on the display
    /// if it does not fit on it, it will only display the first number of digits
    /// when the
    pub fn display_string(&mut self, text_str: &str) {
        let mut pos = 0;
        for txt in text_str.chars() {
            if txt.is_digit(10) {
                self.print_number(txt as u8 - 48, pos);
            } else {
                self.display_char(txt, pos);
            }
            pos += 1;
            if pos == self.number_of_digits as usize {
                break;
            }
        }
    }
}
