mod screens;

use crate::{BoardSpi, DisplayResources};
use defmt::{info, warn};
use embassy_embedded_hal::shared_bus::blocking::spi::SpiDeviceWithConfig;
use embassy_rp::gpio::{Level, Output};
use embassy_sync::{
    blocking_mutex::raw::CriticalSectionRawMutex,
    watch::{Sender, Watch},
};
use embassy_time::Delay;
use embedded_graphics::Drawable;
use icd::Screen;
use mipidsi::{
    interface::SpiInterface,
    models::ILI9341Rgb666,
    options::{ColorOrder, Orientation, Rotation},
};

pub(crate) type UpdateScreenSender = Sender<'static, CriticalSectionRawMutex, Screen, 1>;
pub(crate) static UPDATE_SCREEN: Watch<CriticalSectionRawMutex, Screen, 1> = Watch::new();

#[embassy_executor::task]
pub async fn run(spi: BoardSpi, r: DisplayResources) {
    let mut config = embassy_rp::spi::Config::default();
    config.frequency = 64_000_000;

    let cs = Output::new(r.cs, Level::Low);
    let spi = SpiDeviceWithConfig::new(&spi, cs, config.clone());

    let dc = Output::new(r.dc, Level::Low);
    let rst = Output::new(r.reset, Level::Low);
    let _backlight = Output::new(r.backlight, Level::High);

    let mut buffer = [0_u8; 512];
    let interface = SpiInterface::new(spi, dc, &mut buffer);

    let mut display = mipidsi::Builder::new(ILI9341Rgb666, interface)
        .display_size(240, 320)
        .orientation(
            Orientation::default()
                .rotate(Rotation::Deg270)
                .flip_horizontal(),
        )
        .color_order(ColorOrder::Bgr)
        .reset_pin(rst)
        .init(&mut Delay)
        .expect("display should be initialised");

    {
        let screen = self::screens::SplashScreen {};
        if screen.draw(&mut display).is_err() {
            warn!("Failed to draw splash screen");
        }
    }

    let mut screen_rx = UPDATE_SCREEN
        .receiver()
        .expect("should have a receiver for the update screen watch");

    loop {
        let new_screen = screen_rx.changed().await;

        info!("Drawing screen: {:?}", new_screen);
        if match new_screen {
            Screen::CharacterSelect(s) => {
                self::screens::CharacterSelectScreen::new(s).draw(&mut display)
            }
            Screen::Choices(s) => self::screens::ChoiceScreen::new(s).draw(&mut display),
        }
        .is_err()
        {
            warn!("Failed to draw screen");
        }
    }
}
