use std::thread;

use embedded_graphics::{
    image::{Image, ImageRawLE},
    mono_font::{ascii::FONT_10X20, MonoTextStyleBuilder},
    pixelcolor::BinaryColor,
    prelude::*,
    text::{Baseline, Text},
};

use rppal::{
    gpio::{Gpio, Level},
    spi::{Bus, Mode, SimpleHalSpiDevice, SlaveSelect, Spi},
};
// use ssd1306::{mode::BufferedGraphicsMode, prelude::*, Ssd1306};
use ssd1309::{mode::GraphicsMode, Builder};

use display_interface_spi::SPIInterface;

fn main() {
    let gpio = Gpio::new().unwrap();
    // either 22 or 15 - i think 22
    let dc = gpio.get(25).unwrap().into_output();
    let mut reset = gpio.get(27).unwrap().into_output_low();
    reset.write(Level::High);
    let spi = Spi::new(Bus::Spi0, SlaveSelect::Ss0, 16_000_000, Mode::Mode0).unwrap();
    let spidev = SimpleHalSpiDevice::new(spi);
    let interface = SPIInterface::new(spidev, dc);

    // let mut display = Ssd1306::new(interface, DisplaySize128x64, DisplayRotation::Rotate0)
    //     .into_buffered_graphics_mode();

    // let mut delay = thread::sleep(1);

    let mut disp: GraphicsMode<_> = Builder::new().connect(interface).into();
    // disp.reset(&mut reset &mut
    disp.init().unwrap();
    disp.flush().unwrap();

    let im: ImageRawLE<BinaryColor> =
        ImageRawLE::new(include_bytes!("../images/raw/blush.raw"), 128);

    Image::new(&im, Point::new(0, 0)).draw(&mut disp).unwrap();

    // let text_style = MonoTextStyleBuilder::new()
    //     .font(&FONT_10X20)
    //     .text_color(BinaryColor::On)
    //     .build();

    // Text::with_baseline("(✿˶>﹏<˶)", Point::zero(), text_style, Baseline::Top)
    //     .draw(&mut disp)
    //     .unwrap();

    disp.flush().unwrap();

    println!("Hello, world!");
}
