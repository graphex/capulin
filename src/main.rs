use ht16k33::{Dimming, Display, HT16K33};
use adafruit_alphanum4::{AlphaNum4, AsciiChar, Index};
use shared_bus::*;
use esp_idf_hal::delay::{FreeRtos, BLOCK};
use esp_idf_hal::i2c::*;
use esp_idf_hal::peripherals::Peripherals;
use esp_idf_hal::prelude::*;

use esp_idf_sys as _; // If using the `binstart` feature of `esp-idf-sys`, always keep this module imported

fn main() -> anyhow::Result<()> {
    // It is necessary to call this function once. Otherwise some patches to the runtime
    // implemented by esp-idf-sys might not link properly. See https://github.com/esp-rs/esp-idf-template/issues/71
    esp_idf_sys::link_patches();

    println!("Hello, world!");


    // let freq: Hertz = 20.khz().into();

    let peripherals = Peripherals::take().unwrap();
    let i2c = peripherals.i2c0;
    let sda = peripherals.pins.gpio10;
    let scl = peripherals.pins.gpio8;

    let config = I2cConfig::new().baudrate(1000.kHz().into());
    let i2c = I2cDriver::new(i2c, sda, scl, &config)?;
    let mut i2c_bus = BusManagerSimple::new(i2c);
    
    const DISP_0_I2C_ADDR: u8 = 0x70;
    const DISP_1_I2C_ADDR: u8 = 0x71;
    const DISP_2_I2C_ADDR: u8 = 0x72;

    let mut d0 = HT16K33::new(i2c_bus.acquire_i2c(), DISP_0_I2C_ADDR);
    let mut d1 = HT16K33::new(i2c_bus.acquire_i2c(), DISP_1_I2C_ADDR);
    let mut d2 = HT16K33::new(i2c_bus.acquire_i2c(), DISP_2_I2C_ADDR);

    d0.initialize().expect("Failed to initialize ht16k33");
    d0.set_display(Display::ON).expect("D0 Display would not turn on");
    d0.set_dimming(Dimming::BRIGHTNESS_16_16).expect("Dimming failed");

    d1.initialize().expect("Failed to initialize ht16k33");
    d1.set_display(Display::ON).expect("D1 Display would not turn on");
    d1.set_dimming(Dimming::BRIGHTNESS_16_16).expect("Dimming failed");

    d2.initialize().expect("Failed to initialize ht16k33");
    d2.set_display(Display::ON).expect("D2 Display would not turn on");
    d2.set_dimming(Dimming::BRIGHTNESS_16_16).expect("Dimming failed");

    let mut counter = 0u64;
    loop {
        d0.update_buffer_with_digit(Index::One, (counter / 100000000000 % 10) as u8);
        d0.update_buffer_with_digit(Index::Two, (counter / 10000000000 % 10) as u8);
        d0.update_buffer_with_digit(Index::Three, (counter / 1000000000 % 10) as u8);
        d0.update_buffer_with_digit(Index::Four, (counter / 100000000 % 10) as u8);
        d0.write_display_buffer().unwrap();
        d1.update_buffer_with_digit(Index::One, (counter / 10000000 % 10) as u8);
        d1.update_buffer_with_digit(Index::Two, (counter / 1000000 % 10) as u8);
        d1.update_buffer_with_digit(Index::Three, (counter / 100000 % 10) as u8);
        d1.update_buffer_with_digit(Index::Four, (counter / 10000 % 10) as u8);
        d1.write_display_buffer().unwrap();
        d2.update_buffer_with_digit(Index::One, (counter / 1000 % 10) as u8);
        d2.update_buffer_with_digit(Index::Two, (counter / 100 % 10) as u8);
        d2.update_buffer_with_digit(Index::Three, (counter / 10 % 10) as u8);
        d2.update_buffer_with_digit(Index::Four, (counter % 10) as u8);
        d2.write_display_buffer().unwrap();

        // println!("Showing {}", counter);
        counter = counter + 1;
        // FreeRtos::delay_ms(500);
    }


    // // Sending individual digits
    // d0.update_buffer_with_digit(Index::One, 1);
    // d0.update_buffer_with_digit(Index::Two, 2);
    // d0.update_buffer_with_digit(Index::Three, 3);
    // d0.update_buffer_with_digit(Index::Four, 4);
    //
    // // Sending ascii
    // ht16k33.update_buffer_with_char(Index::One, AsciiChar::new('A'));
    // ht16k33.update_buffer_with_char(Index::Two, AsciiChar::new('B'));
    //
    // // Setting the decimal point
    // ht16k33.update_buffer_with_dot(Index::Two, true);
    //
    // // Formatting a float using the whole display
    // ht16k33.update_buffer_with_float(Index::One, -3.14, 2, 10).unwrap();
    //
    // // Putting a character in front of a float
    // ht16k33.update_buffer_with_char(Index::One, AsciiChar::new('X'));
    // ht16k33.update_buffer_with_float(Index::Two, -3.14, 2, 10).unwrap(); //Display will read "X-3.1"
    //
    // // This will panic because there aren't enough digits to display this number
    // ht16k33.update_buffer_with_float(Index::One, 12345., 0, 10).expect("Oops");
    //
    // Note: none of the above methods actually commit the buffer to the display, call write_display_buffer to actually send it to the display

    Ok(())
}
