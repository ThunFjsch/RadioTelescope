#![deny(unsafe_code)]
#![no_main]
#![no_std]

use panic_halt as _; // Panic handler
use cortex_m_rt::entry;
use stm32f1xx_hal::{pac, prelude::*};

#[entry]
fn main() -> ! {
    // Get access to device peripherals
    let dp = pac::Peripherals::take().unwrap();
    
    // Get access to RCC (Reset and Clock Control)
    let rcc = dp.RCC.constrain();
    
    // Get access to GPIOA (the user LED on Nucleo-F103RB is on PA5)
    let mut gpioa = dp.GPIOA.split();
    
    // Configure PA5 as push-pull output (the built-in LED)
    let mut led = gpioa.pa5.into_push_pull_output(&mut gpioa.crl);
    
    // Get delay provider
    let cp = cortex_m::Peripherals::take().unwrap();
    let clocks = rcc.cfgr.freeze(&mut dp.FLASH.constrain().acr);
    let mut delay = cp.SYST.delay(&clocks);
    
    loop {
        led.set_high();
        delay.delay_ms(500_u16);
        led.set_low();
        delay.delay_ms(500_u16);
    }
}
