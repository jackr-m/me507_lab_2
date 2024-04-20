//! Crate to interface full H-bridge motor drivers

#![deny(missing_docs)]
#![deny(warnings)]

extern crate embedded_hal;

// pub mod ic;

// use core::marker::PhantomData;

// use embedded_hal::digital::v2::OutputPin;
// use embedded_hal::{Pwm, PwmPin};
// use embedded_hal::PwmPin;
use embassy_stm32::timer::simple_pwm::{Ch1, PwmPin};

/// A full H-bridge motor driver
pub struct Motor<IN1, IN2>
where
    IN1: PwmPin,
    IN2: PwmPin,
{
    in1: IN1,
    in2: IN2,
}

impl<IN1, IN2> Motor<IN1, IN2>
where
    IN1: PwmPin,
    IN2: PwmPin,
{
    /// Brakes the motor
    pub fn brake(&mut self) -> &mut Self {
        // self.in1.set_high().expect_err("Error setting pin high");
        // self.in2.set_high().expect_err("Error setting pin high");
        self.in1.disable();
        self.in2.disable();
        self
    }

    /// Makes the motor spin in CounterClockWise direction
    pub fn ccw(&mut self) -> &mut Self {
        self.in1.disable();
        self.in2.disable();
        self
    }

    /// Makes the motor spin in ClockWise direction
    pub fn cw(&mut self) -> &mut Self {
        self.in1.disable();
        self.in2.disable();
        self
    }

    /// Returns the maximum
    pub fn get_max_duty(&mut self) -> IN1::Duty {
        self.in1.get_max_duty()
    }

    /// Changes the motor speed
    pub fn duty(&mut self, duty: IN1::Duty) -> &mut Self {
        self.in1.set_duty(duty);
        self
    }
}

impl<IN1, IN2> Motor<IN1, IN2>
where
    IN1: PwmPin,
    IN2: PwmPin,
{
    /// Creates a new `Motor`
    pub fn tb67(mut in1: IN1, mut in2: IN2) -> Self {
        // initial state: brake
        in1.disable();
        in2.disable();

        Motor {
            in1,
            in2,
        }
    }
}