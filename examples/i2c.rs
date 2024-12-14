// Setup i2c and delays.
// 
//  Usage example 1:
//        in Cargo.toml:  embedded-aht20   = "0.1.3"   
//   
//        use embedded_aht20::{Aht20, DEFAULT_I2C_ADDRESS}; 
//        ...
//        let (i2c1, mut delay, mut delay2) = setup();
//        let mut aht  = Aht20::new(i2c1, DEFAULT_I2C_ADDRESS, &mut delay).expect("sensor initialization failed.");
//        loop {
//           let th = aht.measure().unwrap();   // Read humidity and temperature.
//           delay2.delay(5000.millis()); 
//        }
// 
//  Usage example 2:
//        in Cargo.toml:  aht20-driver     = { version = "2.0.0", default-features = false }
//   
//        use aht20_driver::{AHT20, SENSOR_ADDRESS}; 
//        ...
//        let (i2c1, mut delay, mut delay2) = setup(); 
//        let mut aht20_uninit = AHT20::new(i2c1, SENSOR_ADDRESS);
//        let mut aht = aht20_uninit.init(&mut delay).unwrap();
//        loop {
//           let th = aht.measure(&mut delay).unwrap();  
//           hprintln!("{:.2}C  {:.2}% RH", th.temperature, th.humidity);
//           delay2.delay(5000.millis()); 
//        }
// 
//  Usage example 3:
//        in Cargo.toml:  aht20  = { git = "https://github.com/blueluna/aht20" }
//   
//        use aht20::{Aht20};  
//        ...
//        let (i2c1, mut delay, mut delay2) = setup();
//        let mut aht = Aht20::new(i2c1, &mut delay);
//        loop {
//           let (h, t) = aht.read().unwrap();
//           hprintln!("{:.3}C  {}% RH", t.celsius(), h.rh());
//           delay2.delay(5000.millis()); 
//        }
// 
//  Usage example 4:
//        in Cargo.toml:    embedded-sht3x =  "0.1.0"
//   
//        use  embedded_sht3x::{Repeatability::High, Sht3x, DEFAULT_I2C_ADDRESS}; 
//        ...
//        let (i2c1, mut delay, mut delay2) = setup();
//        let mut sen  = Sht3x::new(i2c1, DEFAULT_I2C_ADDRESS, &mut delay);
//        loop {
//           let th = sen.single_measurement().unwrap();   // Read humidity and temperature.
//           hprintln!("{:.2}C  {:.2}% RH", th.temperature, th.humidity);
//           delay2.delay(5000.millis()); 
//        }
// 
// 
//  Usage example 5:
//        in Cargo.toml:  shtcx = { git = "https://github.com/dbrgn/shtcx-rs" }
//   
//        use  shtcx::{LowPower, PowerMode};
//        ...
//        let (i2c1, mut delay, mut delay2) = setup();
//        let mut sen  = shtcx::shtc3(i2c1);
//        sen.wakeup(&mut delay).expect("Wakeup failed"); 
//        loop {
//           let th = sen.measure(PowerMode::NormalMode, &mut delay)
//                                .expect("Normal mode measurement failed");
//           hprintln!("{:.2}C  {:.2}% RH", th.temperature.as_degrees_celsius(), th.humidity.as_percent());
//           delay2.delay(5000.millis()); 
//        }
// 

#![deny(unsafe_code)]
#![no_std]
#![no_main]

#[cfg(debug_assertions)]
use panic_semihosting as _;

#[cfg(not(debug_assertions))]
use panic_halt as _;

use cortex_m_rt::entry;

use cortex_m_semihosting::hprintln;

use stm32f1xx_hal::{
    pac::{CorePeripherals, Peripherals, I2C1 },
    i2c::{DutyCycle, Mode, BlockingI2c}, 
    timer::{SysTimerExt, TimerExt, SysDelay},
    gpio::GpioExt,
    prelude::*, 
};

use embedded_hal::delay::DelayNs;


fn setup() -> (BlockingI2c<I2C1>, impl DelayNs, SysDelay){

    let cp = CorePeripherals::take().unwrap();
    let dp = Peripherals::take().unwrap();

    let mut flash = dp.FLASH.constrain();
    let rcc = dp.RCC.constrain();
    let clocks = rcc.cfgr.freeze(&mut flash.acr);
    // or
    //let clocks = rcc.cfgr.use_hse(8.MHz()).freeze(&mut flash.acr);

    let mut gpiob = dp.GPIOB.split();
    //let scl = gpiob.pb6; 
    //let sda = gpiob.pb7; 
    //let i2c1 = BlockingI2c::<I2C1>::new(dp.I2C1, (scl, sda),
    //               Mode::Fast {frequency: 400.kHz(), duty_cycle: DutyCycle::Ratio16to9,},
    //               &clocks, 1000, 10, 1000, 1000,);
    // or
    // 
    // add .remap(&mut afio.mapr) to use PB8, PB9 instead 
    let scl = gpiob.pb8.into_alternate_open_drain(&mut gpiob.crh); 
    let sda = gpiob.pb9.into_alternate_open_drain(&mut gpiob.crh); 

    let mut afio = dp.AFIO.constrain();

    //let i2c1 = BlockingI2c::<I2C1>::new(
    //               dp.I2C1
    //               .remap(&mut afio.mapr), 
    //               (scl, sda),
    //               Mode::Fast {frequency: 400.kHz(), duty_cycle: DutyCycle::Ratio16to9,},
    //               &clocks, 1000, 10, 1000, 1000,);
    
    // or
    let i2c1 = dp
        .I2C1
        .remap(&mut afio.mapr) // add this if want to use PB8, PB9 instead
        .blocking_i2c(
            (scl, sda),
            Mode::Fast {frequency: 400.kHz(), duty_cycle: DutyCycle::Ratio16to9, },
            &clocks, 1000, 10, 1000, 1000,
        );
 
    // TIM1 TIM2 TIM3 and TIM4 implement trait Instance needed for these
    //let mut delay = stm32f1xx_hal::timer::FTimerUs::new(dp.TIM2, &clocks).delay();
    // or
    let delay = dp.TIM2.delay_us(&clocks);    

    let delay2 = cp.SYST.delay(&clocks); 
 
    (i2c1, delay, delay2)
}

#[entry]
fn main() -> ! {
    
    let (_i2c1, mut delay, mut delay2) = setup();

    hprintln!("Start the sensor...");
    //    as in example 1 above
    // let mut aht  = Aht20::new(i2c1, DEFAULT_I2C_ADDRESS, &mut delay).expect("sensor initialization failed.");

    delay.delay_ms(500);   // this is a DelayNs

    loop {
        hprintln!("aht.measure()");

        // as in example 1 above
        //let th = aht.measure().unwrap();   // Read humidity and temperature.
        //hprintln!("{:.3}C  {}% RH", th.temperature.celcius(), th.relative_humidity);

        delay2.delay(5000.millis());   // this is a SysDelay
    }
}
