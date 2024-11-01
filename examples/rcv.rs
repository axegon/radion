use radion::Device;
use std::error::Error;
use std::fs::File;
use std::io::Write;

fn main() -> Result<(), Box<dyn Error>> {
    // Open the first RTL-SDR device
    let device = Device::new(0)?;

    // Set the center frequency to 100 MHz
    device.set_center_freq(100_000_000)?;

    // Set the sample rate to 2.048 MS/s
    device.set_sample_rate(2_048_000)?;

    // Enable manual gain control and set the gain to maximum
    device.set_tuner_gain_mode(true)?;
    if let Some(&max_gain) = device.get_tuner_gains()?.last() {
        device.set_tuner_gain(max_gain)?;
    }

    // Reset the buffer before reading
    device.reset_buffer()?;

    // Read 256,000 bytes of IQ data
    let data = device.read_sync(256_000)?;

    // Write the data to a binary file
    let output_file_path = "samples.bin";
    File::create(output_file_path)?.write_all(&data)?;
    println!(
        "Wrote {} bytes of IQ data to {}",
        data.len(),
        output_file_path
    );

    Ok(())
}
