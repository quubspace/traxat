use crate::sqlite::query_position;
use anyhow::{anyhow, Context, Result};
use rppal::{gpio::Gpio, system::DeviceInfo};
use std::thread;
use std::time::Duration;

const GPIO_PINS: [u8; 12] = [6, 13, 19, 26, 9, 11, 0, 5];

pub fn custom_move(option: Vec<usize>) -> Result<()> {
    println!("You have chosen to move your arm to position: {:?}", option);

    for x in &option {
        if *x < 1 {
            return Err(anyhow!("Number of steps cannot be less than one."));
        }
    }

    move_steppers(option)
}

pub fn predefined_move(option: &str) -> Result<()> {
    let position = query_position(&option.to_lowercase())
        .context("No such position. Please choose a valid position name.")?;

    println!(
        "You have chosen to move your arm to named position: {}",
        position.name.to_lowercase()
    );

    Ok(())
}

fn move_steppers(steppers: Vec<usize>) -> Result<()> {
    println!("Moving motor on a {}.", DeviceInfo::new()?.model());

    let mut chunked_pins = GPIO_PINS.chunks(4);

    for steps in steppers {
        let pins_list = &chunked_pins.next().ok_or_else(|| {
            anyhow!("No next stepper! Ensure you did not add an extra step number!")
        })?;
        for x in 0..steps {
            step_pins(x + 1, pins_list)?;
        }
    }

    println!("Done!");

    Ok(())
}

fn step_pins(step_num: usize, pins_list: &[u8]) -> Result<()> {
    let mut last = Gpio::new()?
        .get(*pins_list.last().ok_or_else(|| anyhow!("No last pin!"))?)?
        .into_output();

    last.set_high();
    thread::sleep(Duration::from_millis(1));
    last.set_low();

    drop(last);

    for pair in pins_list.windows(2) {
        let mut last = Gpio::new()?.get(pair[0])?.into_output();
        let mut cur = Gpio::new()?.get(pair[1])?.into_output();

        last.set_high();
        cur.set_high();
        thread::sleep(Duration::from_millis(1));
        last.set_low();
        cur.set_low();

        cur.set_high();
        thread::sleep(Duration::from_millis(1));
        cur.set_low();
    }

    let mut first = Gpio::new()?
        .get(*pins_list.first().ok_or_else(|| anyhow!("No first pin!"))?)?
        .into_output();
    let mut last = Gpio::new()?
        .get(*pins_list.last().ok_or_else(|| anyhow!("No last pin!"))?)?
        .into_output();

    last.set_high();
    first.set_high();
    thread::sleep(Duration::from_millis(1));
    last.set_low();
    first.set_low();

    println!("Step number: {}", step_num);

    Ok(())
}
