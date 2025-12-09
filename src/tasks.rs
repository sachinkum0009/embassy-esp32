//! Task definitions for GPIO operations.
//!
//! This module contains Embassy tasks for handling GPIO input and output operations,
//! including button state monitoring and LED toggling.

use esp_hal::gpio::{Input, Output};
use embassy_time::{Duration, Timer};
use defmt::info;

/// Continuously checks the state of a button and logs its value.
///
/// This task reads the button state every 100 milliseconds and logs whether
/// the button is pressed (high) or not pressed (low) using the `defmt` logging framework.
///
/// # Arguments
///
/// * `btn` - A static reference to a GPIO input pin configured as a button
///
/// # Examples
///
/// ```rust
/// let btn = Input::new(peripherals.GPIO0, Pull::Up);
/// spawner.spawn(check_btn(btn)).unwrap();
/// ```
#[embassy_executor::task]
pub async fn check_btn(btn: Input<'static>) {
    loop {
        let btn_val = btn.is_high();
        if btn_val {
            info!("Button 1 is high");
        } else {
            info!("Button 1 is low");
        }
        Timer::after(Duration::from_millis(100)).await;
    }
}

/// Toggles an LED on and off at regular intervals.
///
/// This task switches the LED state every second, creating a blinking effect.
///
/// # Arguments
///
/// * `led` - A static reference to a GPIO output pin configured to control an LED
///
/// # Examples
///
/// ```rust
/// let led = Output::new(peripherals.GPIO2, Level::Low);
/// spawner.spawn(toggle_led(led)).unwrap();
/// ```
#[embassy_executor::task]
pub async fn toggle_led(mut led: Output<'static>) {
    loop {
        led.toggle();
        Timer::after(Duration::from_secs(1)).await;
    }
}
