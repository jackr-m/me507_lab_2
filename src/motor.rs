//! This module provides a simple interface for controlling a motor using a PWM signal for the Embassy STM32 framework.

// Path: src/motor.rs

use embassy_stm32::gpio::{AnyPin, Pin};
use embassy_stm32::timer::simple_pwm::SimplePwm;
use embassy_stm32::timer::Channel;

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
pub struct Motor<IN1, IN2, PWM> {
    in1: IN1,
    in2: IN2,
    pwm: PWM,
    current_drive_command: DriveCommand,
}

impl<IN1, IN2, PWM> Motor<IN1, IN2, PWM>
where
    IN1: Pin,
    IN2: Pin,
    PWM: SimplePwm,
{
    /// Note: pins must correspond to channels of the same timer
    pub fn new(in1: IN1, in2: IN2) -> Result<Motor<IN1, IN2>, MotorError<IN1, IN2>> {
        let mut motor = Motor {
            in1,
            in2,
            current_drive_command: DriveCommand::Stop,
        };

        motor.drive(motor.current_drive_command)?;

        Ok(motor)
    }
    
    /// Drive with the defined speed (or brake or stop the motor).
    #[allow(clippy::type_complexity)]
    pub fn drive(
        &mut self,
        drive_command: DriveCommand,
    ) -> Result<(), MotorError<IN1, IN2>> {
        let speed = match drive_command {
            DriveCommand::Forward(s) | DriveCommand::Backward(s) => s,
            _ => 0,
        };

        if speed > 100 {
            return Err(MotorError::InvalidSpeed);
        }

        match drive_command {
            DriveCommand::Forward(_) => {
                self.in1.set_high().map_err(MotorError::In1Error)?;
                self.in2.set_low().map_err(MotorError::In2Error)?;
            }
            DriveCommand::Backward(_) => {
                self.in1.set_low().map_err(MotorError::In1Error)?;
                self.in2.set_high().map_err(MotorError::In2Error)?;
            }
            DriveCommand::Brake => {
                self.in1.set_high().map_err(MotorError::In1Error)?;
                self.in2.set_high().map_err(MotorError::In2Error)?;
            }
            DriveCommand::Stop => {
                self.in1.set_low().map_err(MotorError::In1Error)?;
                self.in2.set_low().map_err(MotorError::In2Error)?;
            }
        }

        #[cfg(feature = "defmt")]
        defmt::debug!("driving {} with speed {}", drive_command, speed);

        self.pwm
            .set_duty_cycle_percent(speed)
            .map_err(MotorError::PwmError)?;

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