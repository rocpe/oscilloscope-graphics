//! # PWM Blink Example
//!
//! If you have an LED connected to pin 25, it will fade the LED using the PWM
//! peripheral.
//!
//! It may need to be adapted to your particular board layout and/or pin assignment.
//!
//! See the `Cargo.toml` file for Copyright and license details.

#![no_std]
#![no_main]

use display::Display;
// Ensure we halt the program on panic (if we don't mention this crate it won't
// be linked)
use panic_halt as _;

// Alias for our HAL crate
use rp2040_hal as hal;

// Some traits we need
use embedded_hal::pwm::SetDutyCycle;
use rp2040_hal::clocks::Clock;

// A shorter alias for the Peripheral Access Crate, which provides low-level
// register access
use hal::pac;

mod display;

/// The linker will place this boot block at the start of our program image. We
/// need this to help the ROM bootloader get our code up and running.
/// Note: This boot block is not necessary when using a rp-hal based BSP
/// as the BSPs already perform this step.
#[link_section = ".boot2"]
#[used]
pub static BOOT2: [u8; 256] = rp2040_boot2::BOOT_LOADER_GENERIC_03H;

/// The minimum PWM value (i.e. LED brightness) we want
const LOW: u16 = 0;

/// The maximum PWM value (i.e. LED brightness) we want
const HIGH: u16 = 20000;

/// External high-speed crystal on the Raspberry Pi Pico board is 12 MHz. Adjust
/// if your board has a different frequency
const XTAL_FREQ_HZ: u32 = 12_000_000u32;

/// Entry point to our bare-metal application.
///
/// The `#[rp2040_hal::entry]` macro ensures the Cortex-M start-up code calls this function
/// as soon as all global variables and the spinlock are initialised.
///
/// The function configures the RP2040 peripherals, then fades the LED in an
/// infinite loop.
#[rp2040_hal::entry]
fn main() -> ! {
    // Grab our singleton objects
    let mut pac = pac::Peripherals::take().unwrap();
    let core = pac::CorePeripherals::take().unwrap();

    // Set up the watchdog driver - needed by the clock setup code
    let mut watchdog = hal::Watchdog::new(pac.WATCHDOG);

    // Configure the clocks
    //
    // The default is to generate a 125 MHz system clock
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

    // The single-cycle I/O block controls our GPIO pins
    let sio = hal::Sio::new(pac.SIO);

    // Set the pins up according to their function on this particular board
    let pins = hal::gpio::Pins::new(
        pac.IO_BANK0,
        pac.PADS_BANK0,
        sio.gpio_bank0,
        &mut pac.RESETS,
    );

    // The delay object lets us wait for specified amounts of time (in
    // milliseconds)
    let mut delay = cortex_m::delay::Delay::new(core.SYST, clocks.system_clock.freq().to_Hz());

    // Init PWMs
    let mut pwm_slices = hal::pwm::Slices::new(pac.PWM, &mut pac.RESETS);

    // Configure PWM1
    let pwm = &mut pwm_slices.pwm1;
    pwm.set_ph_correct();
    pwm.set_top(HIGH);
    pwm.enable();

    // Output channel A on PWM1 to GPIO 2
    let channel_x = &mut pwm.channel_a;
    channel_x.output_to(pins.gpio2);

    // Output channel B on PWM1 to GPIO 3
    let channel_y = &mut pwm.channel_b;
    channel_y.output_to(pins.gpio3);

    channel_x.set_duty_cycle(LOW).unwrap();
    channel_y.set_duty_cycle(LOW).unwrap();

    let range = (LOW..HIGH).filter(|x| x % 6 == 0);

    let time = 1;

    channel_y.set_duty_cycle(10000).unwrap();

    let mut display = Display::new(
        channel_x,
        channel_y,
        -4.,
        4.,
        -3.,
        3.,
    );

    loop {
        // for i in range.clone() {
        //     let _ = channel_x.set_duty_cycle(i);
        //     delay.delay_us(time);
        // }
        // for i in range.clone() {
        //     let _ = channel_y.set_duty_cycle(i);
        //     delay.delay_us(time);
        // }
        // for i in range.clone().rev() {
        //     let _ = channel_x.set_duty_cycle(i);
        //     delay.delay_us(time);
        // }
        // for i in range.clone().rev() {
        //     let _ = channel_y.set_duty_cycle(i);
        //     delay.delay_us(time);
        // }

        
        // channel_y.set_duty_cycle(0).unwrap();
        // delay.delay_ms(1000);
        // channel_y.set_duty_cycle(5000).unwrap();
        // delay.delay_ms(1000);
        // channel_y.set_duty_cycle(10000).unwrap();
        // delay.delay_ms(1000);
        // channel_y.set_duty_cycle(15000).unwrap();
        // delay.delay_ms(1000);
        // channel_y.set_duty_cycle(20000).unwrap();
        // delay.delay_ms(1000);

        // for i in 0..=100 {
        //     display.set_position(-1., 1.).unwrap();
        // }


        display.set_position(-1., 1.).unwrap();
        delay.delay_ms(100);
        display.set_position(1., 1.).unwrap();
        delay.delay_ms(100);
        display.set_position(1., -1.).unwrap();
        delay.delay_ms(100);
        display.set_position(-1., -1.).unwrap();
        delay.delay_ms(100);
    }
}