use std::thread;

mod emote;

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

use clap::{command, Parser, Subcommand};

#[derive(Parser)]
struct Cli {
    #[command(subcommand)]
    emote: emote::Emotes,
}

fn main() {
    let cli = Cli::parse();

    let gpio = Gpio::new().unwrap();
    // either 22 or 15 - i think 22
    let dc = gpio.get(25).unwrap().into_output();
    let mut reset = gpio.get(27).unwrap().into_output_low();
    reset.write(Level::High);
    let spi = Spi::new(Bus::Spi0, SlaveSelect::Ss0, 16_000_000, Mode::Mode0).unwrap();
    let spidev = SimpleHalSpiDevice::new(spi);
    let interface = SPIInterface::new(spidev, dc);

    let mut disp: GraphicsMode<_> = Builder::new().connect(interface).into();
    // disp.reset(&mut reset &mut
    disp.init().unwrap();
    disp.flush().unwrap();

    let blush: ImageRawLE<BinaryColor> =
        ImageRawLE::new(include_bytes!("../images/raw/blush.raw"), 128);

    let innocent: ImageRawLE<BinaryColor> =
        ImageRawLE::new(include_bytes!("../images/raw/innocent.raw"), 128);

    let im = match cli.emote {
        emote::Emotes::Blush => blush,
        emote::Emotes::Innocent => innocent,
    };

    Image::new(&im, Point::new(0, 0)).draw(&mut disp).unwrap();

    disp.flush().unwrap();
}
