#![no_std]
#![no_main]
#![deny(
    clippy::mem_forget,
    reason = "mem::forget is generally not safe to do with esp_hal types, especially those \
    holding buffers for the duration of a data transfer."
)]

use esp_hal::{clock::CpuClock, gpio::Input, gpio::InputConfig, gpio::Output, gpio::OutputConfig, gpio::Level, gpio::Pull};
use esp_hal::timer::timg::TimerGroup;

use esp_println as _;
use defmt::info;


// use esp_hal::io::IO;

use embassy_executor::Spawner;
use embassy_time::{Duration, Timer};

use esp_backtrace as _;

extern crate alloc;

// import tasks
use my_esp_project::tasks::{check_btn, toggle_led};

// This creates a default app-descriptor required by the esp-idf bootloader.
// For more information see: <https://docs.espressif.com/projects/esp-idf/en/stable/esp32/api-reference/system/app_image_format.html#application-description>
esp_bootloader_esp_idf::esp_app_desc!();

#[esp_hal_embassy::main]
async fn main(spawner: Spawner) {
    // Wi-Fi Credentials
    let SSID = "abc";
    let PWD = "1234567890";

    let config = esp_hal::Config::default().with_cpu_clock(CpuClock::max());
    let peripherals = esp_hal::init(config);

    esp_alloc::heap_allocator!(size: 64 * 1024);

    let timer0 = TimerGroup::new(peripherals.TIMG1);
    esp_hal_embassy::init(timer0.timer0);

    info!("Embassy initialized!");

    let rng = esp_hal::rng::Rng::new(peripherals.RNG);
    let timer1 = TimerGroup::new(peripherals.TIMG0);
    let wifi_init = esp_wifi::init(timer1.timer0, rng)
        .expect("Failed to initialize WIFI/BLE controller");
    let (mut _wifi_controller, _interfaces) = esp_wifi::wifi::new(&wifi_init, peripherals.WIFI)
        .expect("Failed to initialize WIFI controller");

    let output_config = OutputConfig::default();
    let mut led = Output::new(peripherals.GPIO4, Level::High, output_config);
    led.set_high();

    let input_config = InputConfig::default().with_pull(Pull::Down);
    let btn1 = Input::new(peripherals.GPIO5, input_config);

    spawner.spawn(check_btn(btn1)).unwrap();
    spawner.spawn(toggle_led(led)).unwrap();

    loop {
        Timer::after(Duration::from_secs(1)).await;
    }

}
