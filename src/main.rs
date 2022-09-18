#![no_main]
#![no_std]

use panic_halt as _;

use cortex_m_rt::entry;
use stm32f4xx_hal as hal;

use crate::hal::{pac, prelude::*};


#[entry]
fn main() -> ! {
    let dp = pac::Peripherals::take().unwrap();

    let gpioc = dp.GPIOC.split();
    let gpiod = dp.GPIOD.split();


    let rcc = dp.RCC.constrain();
    let clocks = rcc.cfgr.use_hse(25.MHz()).freeze();
    let mut delay = dp.TIM1.delay_ms(&clocks);


    let mut led1 = gpioc.pc15.into_push_pull_output();
    let mut led2 = gpiod.pd2.into_push_pull_output();
    // define RX/TX pins


    loop {
        // print some value every 500 ms, value will overflow after 255

            led1.set_high();
            led2.set_high();
    

        delay.delay(2.secs());

            led1.set_low();
            led2.set_low();
        delay.delay(2.secs());
    }
    
}