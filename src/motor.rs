//! This module provides a simple interface for controlling a motor using a PWM signal for the Embassy STM32 framework.

// Path: src/motor.rs

use embassy_stm32::gpio::{AnyPin, OutputType};
use embassy_stm32::time::khz;
use embassy_stm32::timer::simple_pwm::{PwmPin, SimplePwm};
use embassy_stm32::timer::Channel;
use embassy_stm32::timer;
use embedded_hal::Pwm;


pub trait PwmTrait: timer::CaptureCompare16bitInstance {}
pub type PwmImpl<TimPeri> = SimplePwm<'static, TimPeri>;

pub type PwmMotor = SimplePwm<'static, embassy_stm32::peripherals::TIM1>;

/// Defines errors which can happen when calling [`Motor::drive()`].
pub enum MotorError<IN1Error, IN2Error> {
    /// An invalid speed has been defined. The speed must be given as a percentage value between 0 and 100 to be valid.
    InvalidSpeed,
    /// An error in setting the output of the IN1 pin
    In1Error(IN1Error),
    /// An error in setting the output of the IN2 pin
    In2Error(IN2Error),
}

/// Defines the possible drive commands.
pub enum DriveCommand {
    /// Drive forward with the defined speed (in percentage)
    Forward(u8),
    /// Drive backward with the defined speed (in percentage)
    Backward(u8),
    /// Actively brake
    Brake,
    /// Coast, i.e. stop but don't actively brake.
    Stop,
}

/// Represents a single motor.
pub struct Motor {
    pwm: PwmMotor,
    current_drive_command: DriveCommand,
}

impl Motor {
    /// Note: pins must correspond to channels of the same timer
    pub fn new(in_1: AnyPin, in_2: AnyPin) -> Self {
        let pwm_ch_1 = PwmPin::new_ch1(in_1, OutputType::PushPull);
        let pwm_ch_2 = PwmPin::new_ch2(in_2, OutputType::PushPull);
        let pwm_motor = SimplePwm::new(PwmTrait, pwm_ch_1, pwm_ch_2, None, None, khz(50), Default::default());
        let mut motor = Motor {
            pwm: pwm_motor,
            current_drive_command: DriveCommand::Stop,
        };
        
        motor.drive(motor.current_drive_command)
    }
    
    /// Drive with the defined speed (or brake or stop the motor).
    pub fn drive(&mut self, drive_command: DriveCommand, ) -> Result<()> {
        let speed = match drive_command {
            DriveCommand::Forward(s) | DriveCommand::Backward(s) => s,
            _ => 0,
        };

        if speed > 100 {
            return Err(MotorError::InvalidSpeed);
        }

        match drive_command {
            DriveCommand::Forward(_) => {
                
            }
            DriveCommand::Backward(_) => {
                
            }
            DriveCommand::Brake => {
                
            }
            DriveCommand::Stop => {
                
            }
        }

        #[cfg(feature = "defmt")]
        defmt::debug!("driving {} with speed {}", drive_command, speed);

        self.pwm.set_duty_cycle_percent(speed);

        self.current_drive_command = drive_command;

        Ok(())
    }
}

/*pub fn motor_control(pwm: &mut SimplePwm<T>, mut duty: i16) {
    let max = pwm.get_max_duty();
    // let duty = duty as u16;
    
    // Clamp duty if it goes over 100 percent
    if duty > 100 {
        duty = 100;
    } else if duty < -100 {
        duty = -100;
    }
    
    if duty == 0 {
        pwm.set_duty(Channel::Ch1, 0);
        pwm.set_duty(Channel::Ch2, 0);
    } else if duty <= 0 {
        pwm.set_duty(Channel::Ch1, 0);
        pwm.set_duty(Channel::Ch2, ((duty.abs() as u16)/100)*max);
    } else {
        pwm.set_duty(Channel::Ch1, ((duty as u16)/100)*max);
        pwm.set_duty(Channel::Ch2, 0);
    }
}*/