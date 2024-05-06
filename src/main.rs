#![no_std]
#![no_main]

// Ensure we halt the program on panic
use panic_halt as _;

use rp2040_hal as hal;
use rp2040_hal::clocks::Clock;
use hal::pac;

use libm::{cosf, sinf};

use display::Display;

#[link_section = ".boot2"]
#[used]
pub static BOOT2: [u8; 256] = rp2040_boot2::BOOT_LOADER_GENERIC_03H;

const TOP: u16 = 1024;

const XTAL_FREQ_HZ: u32 = 12_000_000u32;

mod display;

#[rp2040_hal::entry]
fn main() -> ! {
    let mut pac = pac::Peripherals::take().unwrap();
    let core = pac::CorePeripherals::take().unwrap();

    let mut watchdog = hal::Watchdog::new(pac.WATCHDOG);

    // Should by default be a 125 MHz system clock? - measurements suggest it is equal to ~65 MHz
    let clocks = hal::clocks::init_clocks_and_plls(
        XTAL_FREQ_HZ,
        pac.XOSC,
        pac.CLOCKS,
        pac.PLL_SYS,
        pac.PLL_USB,
        &mut pac.RESETS,
        &mut watchdog,
    )
    .unwrap();

    let sio = hal::Sio::new(pac.SIO);
    let pins = hal::gpio::Pins::new(
        pac.IO_BANK0,
        pac.PADS_BANK0,
        sio.gpio_bank0,
        &mut pac.RESETS,
    );

    let mut delay = cortex_m::delay::Delay::new(core.SYST, clocks.system_clock.freq().to_Hz());

    // Init PWMs
    let mut pwm_slices = hal::pwm::Slices::new(pac.PWM, &mut pac.RESETS);

    let pwm = &mut pwm_slices.pwm1;
    pwm.set_ph_correct();
    pwm.set_top(TOP);
    pwm.enable();

    let channel_x = &mut pwm.channel_a;
    channel_x.output_to(pins.gpio2);

    let channel_y = &mut pwm.channel_b;
    channel_y.output_to(pins.gpio3);

    // Init display
    let mut display = Display::new(channel_x, channel_y, -4., 4., -3., 3.);

    let mut t = 0.0;
    loop {
        let sin = sinf(t);
        let (x, y) = (
            2. * sin * sin * sin,
            (13. * cosf(t) - 5. * cosf(2. * t) - 2. * cosf(3. * t)) * 0.125,
        );
        display.set_position(x, y).unwrap();
        delay.delay_us(1);
        t += 0.1;
    }
}
