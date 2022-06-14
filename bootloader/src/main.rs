#![no_main]
#![no_std]
#![allow(deprecated)]
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
use mcu::gpio::gpiod::{PD12};
use mcu::delay::Delay;

use cortex_m::peripheral::Peripherals;

struct Leds{
    green :  PD12<gpio::Output<gpio::PushPull>>,
}



pub const FLASH_PAGE_SIZE : u32 = 131072;   // 1 sector size = 128KB   
pub const STACK_LOW       : u32 = 0x20_000_000;
pub const STACK_UP        : u32 = 0x20_020_000;
pub const RB_HDR_SIZE     : u32 = 0x100;
pub const BASE_ADDR       : u32 = 0x08040000;   //  sector 6 starting address
pub const VTR_TABLE_SIZE  : u32 = 0x100;
pub const FW_RESET_VTR    : u32 = BASE_ADDR+4;// + RB_HDR_SIZE + VTR_TABLE_SIZE + 1;

const SCB_AIRCR_VECTKEY: u32 = 0x05FA << 16;
const SCB_AIRCR_PRIGROUP_MASK: u32 = 0x7 << 8;
const SCB_AIRCR_SYSRESETREQ: u32 = 1 << 2;

#[inline]
pub fn nvic_systemreset() -> ! {
// Initiate a system reset request to reset the MCU
     let scb = mcu::pac::SCB::ptr();
        cortex_m::asm::dsb();
        unsafe {
            (*scb).aircr.modify(
                |r| {
                    SCB_AIRCR_VECTKEY | // otherwise the write is ignored
            r & SCB_AIRCR_PRIGROUP_MASK | // keep priority group unchanged
            SCB_AIRCR_SYSRESETREQ
                }, // set the bit
               )
        };
        cortex_m::asm::dsb();
        loop {
            // wait for the reset
            cortex_m::asm::nop(); // avoid rust-lang/rust#28728
        }
    }

    pub fn boot_from(fw_base_address: usize) -> ! {
        let address = fw_base_address as u32;
      let scb = mcu::pac::SCB::ptr();
      unsafe {
      let sp = *(address as *const u32);
      let rv = *((address + 4) as *const u32);
      //USER_RESET = Some(core::mem::transmute(rv));
      let jump_vector = core::mem::transmute::<usize, extern "C" fn() -> !>(rv as usize);
      (*scb).vtor.write(address);
      cortex_m::register::msp::write(sp);
      jump_vector();
   
      }
      loop{}
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
        green  : gpiod.pd12.into_push_pull_output(),
    };

    let mut count =5;
     while count > 0
     {
        leds.green.set_high();
        delay.delay_ms(1000_u16);
        leds.green.set_low();
        delay.delay_ms(1000_u16);
        count = count -1;
     }




   boot_from(0x08020000);
}
 
loop{};
}



