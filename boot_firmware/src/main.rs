#![no_main]
#![no_std]
//#![allow(deprecated)]
extern crate cortex_m;
extern crate cortex_m_rt;
extern crate panic_halt;

extern crate stm32f4xx_hal as mcu;
extern crate embedded_hal as hal;


use cortex_m_rt::entry;

use defmt_rtt as _;
use mcu::prelude::*;
use mcu::stm32;
use mcu::gpio;
use mcu::gpio::gpiod::{PD13};
use mcu::delay::Delay;

use cortex_m::peripheral::Peripherals;



struct Leds{
    orange:  PD13<gpio::Output<gpio::PushPull>>,

}
 
#[entry]
fn main() -> !{

    if let (Some(peri) ,Some(cortex_peri ))=(stm32::Peripherals::take(),Peripherals::take())
    {
   
    //1. to Enable the clock

    let  rcc = peri.RCC.constrain();

    let clocks1 = rcc.cfgr.sysclk(84.mhz()).freeze();
    let mut delay  = Delay::new(cortex_peri.SYST,&clocks1);

    let gpiod =  peri.GPIOD.split();  
    let mut leds = Leds{
        orange : gpiod.pd13.into_push_pull_output(),
    
    };
    let mut count =5;
    while count > 0
    {
    leds.orange.set_high();
    delay.delay_ms(1000_u16);
    leds.orange.set_low();
    delay.delay_ms(1000_u16);
    count = count -1;
    }

}

loop{};
}



