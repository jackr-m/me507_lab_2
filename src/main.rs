#![no_std]
#![no_main]
#![feature(type_alias_impl_trait)]

mod motor;

use defmt::*; // if info!() or other debug macros are used
use embassy_executor::Spawner;
use embassy_time::Timer;
use embassy_stm32::gpio::{AnyPin, Level, Output, OutputType, Pin, Speed};
// use embassy_stm32::gpio::{Input, Pull};
use embassy_stm32::time::khz;
use embassy_stm32::timer::simple_pwm::{PwmPin, SimplePwm};
use embassy_stm32::timer::{Channel, OutputPolarity};

//noinspection RsUnusedImport
use {defmt_rtt as _, panic_probe as _};

// use crate::motor;


#[embassy_executor::main]
/// Main function, blinks an LED for 200ms on, 300ms off, and prints the current loop number to the console.
async fn main(spawner: Spawner) {
    // Hardware objects
    let p = embassy_stm32::init(Default::default());

    let ch1 = PwmPin::new_ch1(p.PA8, OutputType::PushPull);
    let ch2 = PwmPin::new_ch2(p.PA9, OutputType::PushPull);
    let ch3 = PwmPin::new_ch3(p.PA10, OutputType::PushPull);
    let ch4 = PwmPin::new_ch4(p.PA11, OutputType::PushPull);
    let mut pwm = SimplePwm::new(p.TIM1, Some(ch1), Some(ch2), Some(ch3), Some(ch4), khz(20), Default::default());
    
    // let mut led = Output::new(p.PC13, Level::High, Speed::VeryHigh);
    
    // Variables
    let max = pwm.get_max_duty(); // equates to 65416
    // let mut duty = (max as f32 * (200.0/(200.0+300.0))) as u16; // equates to 25806
    let mut duty: i16 = 20;

    info!("Max Duty Cycle: {}", max);

    pwm.set_polarity(Channel::Ch1, OutputPolarity::ActiveLow); // inverted PWM
    pwm.set_polarity(Channel::Ch2, OutputPolarity::ActiveLow);
    pwm.set_polarity(Channel::Ch3, OutputPolarity::ActiveLow);
    pwm.set_polarity(Channel::Ch4, OutputPolarity::ActiveLow);
    pwm.enable(Channel::Ch1);
    pwm.enable(Channel::Ch2);
    pwm.enable(Channel::Ch3);
    pwm.enable(Channel::Ch4);
    
    info!("Starting Program...");
    // spawner.spawn(blinky(p.PC13.degrade(), 10.0/(duty.unsigned_abs() as f32))).unwrap();
    
    //loop {

        // Clamp duty if it goes over 100 percent
        if duty > 100 {
            duty = 100;
        } else if duty < -100 {
            duty = -100;
        }
    
        info!("Duty: {}", duty);
    
        if duty == 0 {
            pwm.set_duty(Channel::Ch1, 0);
            pwm.set_duty(Channel::Ch2, 0);
        } else if duty <= 0 {
            pwm.set_duty(Channel::Ch1, 0);
            pwm.set_duty(Channel::Ch2, ((duty as i32).unsigned_abs()*(max as u32)/100) as u16);
        } else {
            info!("Max Duty: {}", max);
            info!("PWM Duty: {}", ((duty as i32).unsigned_abs()*(max as u32)/100) as u16);
            pwm.set_duty(Channel::Ch1, ((duty as i32).unsigned_abs()*(max as u32)/100) as u16);
            pwm.set_duty(Channel::Ch2, 0);
        }
        
        // duty = (max as f32 * (counter as f32 / 100.0)) as u16;
        // pwm.set_duty(Channel::Ch1, duty);
        // pwm.set_duty(Channel::Ch2, duty);
        // Timer::after_millis(10).await;
        
    //}

    Timer::after_secs(5).await;
    pwm.disable(Channel::Ch1);
    pwm.disable(Channel::Ch2);
    pwm.disable(Channel::Ch3);
    pwm.disable(Channel::Ch4);
    info!("Program Ended, stopping motors.");
    
    loop {
        Timer::after_millis(10).await;
    }
    
    // pwm.disable(Channel::Ch1);
    // pwm.disable(Channel::Ch2);
    // pwm.disable(Channel::Ch3);
    // pwm.disable(Channel::Ch4);
}

#[embassy_executor::task]
async fn blinky(p: AnyPin, speed: f32) {
    let mut led = Output::new(p, Level::Low, Speed::Low);

    info!("Speed: {}", speed);

    info!("LED time: {}", (speed*1000.0/2.0) as u64);

    loop {
        led.set_low();
        Timer::after_millis((speed*1000.0/2.0) as u64).await;
        led.set_high();
        Timer::after_millis((speed*1000.0/2.0) as u64).await;
    }
}