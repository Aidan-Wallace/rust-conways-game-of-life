use rppal::gpio::Gpio;
use std::{error::Error, thread, time::Duration};

fn main() -> Result<(), Box<dyn Error>> {
    println!("Hello, world!");

    let mut pin = Gpio::new()?.get(26)?.into_output();

    pin.set_high();
    thread::sleep(Duration::from_millis(500));
    pin.set_low();

    Ok(())
}
