#![no_std]
#![no_main]

mod buttons;
mod display;
mod rpc;

use assign_resources::assign_resources;
use core::cell::RefCell;
use defmt::info;
use embassy_executor::Spawner;
use embassy_sync::blocking_mutex::{raw::NoopRawMutex, Mutex};
use peek_o_display_bsp::peripherals::{self, PeekODisplay};
use portable_atomic as _;
use {defmt_rtt as _, panic_probe as _};

assign_resources! {
    board_spi: BoardSpiResources {
        spi: SPI0,
        clk: SPI_CLK,
        mosi: SPI_MOSI,
        miso: SPI_MISO,
    }
    display: DisplayResources {
        cs: DISPLAY_CS,
        dc: DISPLAY_DC,
        reset: DISPLAY_RESET,
        backlight: DISPLAY_BACKLIGHT,
    }
    buttons: ButtonResources {
        fn_1: PIN_16,
        fn_2: PIN_17,
        fn_3: PIN_18,
        end_conversation: PIN_19,
    }
    rpc: RpcResources {
        usb: USB,
    }
    led: LedResources {
        led: PIN_25,
    }
}

type BoardSpi = Mutex<
    NoopRawMutex,
    RefCell<embassy_rp::spi::Spi<'static, peripherals::SPI0, embassy_rp::spi::Blocking>>,
>;

#[embassy_executor::main]
async fn main(spawner: Spawner) {
    let p = PeekODisplay::default();
    let r = split_resources!(p);

    info!("Hello, world!");

    let rpc_sender = crate::rpc::init(r.rpc, spawner);

    let mut config = embassy_rp::spi::Config::default();
    config.phase = embassy_rp::spi::Phase::CaptureOnSecondTransition;
    config.polarity = embassy_rp::spi::Polarity::IdleHigh;

    let spi = embassy_rp::spi::Spi::new_blocking(
        r.board_spi.spi,
        r.board_spi.clk,
        r.board_spi.mosi,
        r.board_spi.miso,
        config,
    );
    let spi_bus: BoardSpi = Mutex::new(RefCell::new(spi));

    spawner.must_spawn(display::run(spi_bus, r.display));
    spawner.must_spawn(buttons::run(r.buttons, rpc_sender));
}
