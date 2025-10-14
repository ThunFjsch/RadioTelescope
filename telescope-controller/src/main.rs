#![allow(clippy::empty_loop)]
#![deny(unsafe_code)]
#![no_main]
#![no_std]

use core::fmt::Write;
use cortex_m_rt::entry;
use debouncr::{debounce_3, Edge};
use panic_halt as _;
use stm32f1xx_hal::{
    pac::{self},
    prelude::*,
    serial::{Config, Serial},
};

// For Debug error output
use rtt_target::{rprintln, rtt_init_print};

#[entry]
fn main() -> ! {
    rtt_init_print!();

    let peri = pac::Peripherals::take().unwrap();
    let rcc = peri.RCC.constrain();
    let mut flash = peri.FLASH.constrain();
    let mut afio = peri.AFIO.constrain();

    //let mut gpiob = peri.GPIOB.split();
    let mut gpioa = peri.GPIOA.split();
    let mut gpioc = peri.GPIOC.split();

    let tx = gpioa.pa2.into_alternate_push_pull(&mut gpioa.crl);
    let rx = gpioa.pa3;

    let clocks = rcc.cfgr.use_hse(8.MHz()).freeze(&mut flash.acr);

    let mut serial = Serial::new(
        peri.USART2,
        (tx, rx),
        &mut afio.mapr,
        Config::default().baudrate(115200.bps()),
        &clocks,
    );

    let mut std_delay = 70_000_i32;
    let mut led = gpioa.pa5.into_push_pull_output(&mut gpioa.crl);
    led.set_low();
    let button = gpioc.pc13.into_pull_up_input(&mut gpioc.crh);
    let mut value: u8 = 0;
    let mut debouncer = debounce_3(false);

    loop {
        for _i in 1..std_delay {
            if debouncer.update(button.is_low()) == Some(Edge::Falling) {
                let result = writeln!(serial, "Button Press {:02}\r", value);
                if result.is_err() {
                    rprintln!("{:?}\r", result.err())
                }

                rprintln!("{:?}\r", result);
                value = value.wrapping_add(1);
                std_delay = std_delay - 30_000_i32;
                if std_delay < 10_000 {
                    std_delay = 70_000_i32;
                }
                break;
            }
        }
        led.toggle();
    }
}
