use crate::error::{Error, Result};
use crate::ffi::*;
use crate::hw_info::HwInfo;
use crate::tuner::RTLSDRTuner;
use crate::utils::{
    parse_string_descriptors, serialize_string_descriptors, EEPROM_SIZE, STR_OFFSET_START,
};
use std::ffi::CStr;
use std::os::raw::{c_char, c_int, c_void};
use std::ptr;

pub struct Device {
    dev: *mut RTLSDRDevT,
}

impl Device {
    /// Open a RTL-SDR device by index.
    ///
    /// # Arguments
    ///
    /// * `index` - The index of the device to open.
    ///
    /// # Returns
    ///
    /// A new `Device` instance if successful, otherwise an `Error`.
    pub fn new(index: u32) -> Result<Self> {
        unsafe {
            let mut dev: *mut RTLSDRDevT = ptr::null_mut();
            let err = rtlsdr_open(&mut dev, index);
            if err == 0 {
                Ok(Device { dev })
            } else {
                Err(Error::from(err))
            }
        }
    }

    /// Get the number of available devices.
    ///
    /// # Returns
    ///
    /// The number of available devices.
    pub fn get_device_count() -> u32 {
        unsafe { rtlsdr_get_device_count() }
    }

    /// Get the name of the device by index.
    ///
    /// # Arguments
    ///
    /// * `index` - The index of the device.
    ///
    /// # Returns
    ///
    /// The name of the device if successful, otherwise `None`.
    pub fn get_device_name(index: u32) -> Option<String> {
        unsafe {
            let name_ptr = rtlsdr_get_device_name(index);
            if name_ptr.is_null() {
                None
            } else {
                Some(CStr::from_ptr(name_ptr).to_string_lossy().into_owned())
            }
        }
    }

    /// Get the USB strings of the device.
    ///
    /// # Returns
    ///
    /// The manufacturer, product, and serial strings of the device.
    pub fn get_device_usb_strings(&self) -> Result<(String, String, String)> {
        let mut m: [c_char; 256] = [0; 256];
        let mut p: [c_char; 256] = [0; 256];
        let mut s: [c_char; 256] = [0; 256];
        let ret = unsafe {
            rtlsdr_get_device_usb_strings(self.dev, m.as_mut_ptr(), p.as_mut_ptr(), s.as_mut_ptr())
        };
        if ret == 0 {
            let manufact = unsafe { CStr::from_ptr(m.as_ptr()) }
                .to_string_lossy()
                .into_owned();
            let product = unsafe { CStr::from_ptr(p.as_ptr()) }
                .to_string_lossy()
                .into_owned();
            let serial = unsafe { CStr::from_ptr(s.as_ptr()) }
                .to_string_lossy()
                .into_owned();
            Ok((manufact, product, serial))
        } else {
            Err(Error::from(ret))
        }
    }

    /// Get the index of the device by serial number.
    ///
    /// # Arguments
    ///
    /// * `serial` - The serial number of the device.
    ///
    /// # Returns
    ///
    /// The index of the device if successful, otherwise an `Error`.
    pub fn get_index_by_serial(serial: &str) -> Result<i32> {
        let serial = std::ffi::CString::new(serial).unwrap();
        let ret = unsafe { rtlsdr_get_index_by_serial(serial.as_ptr()) };
        if ret >= 0 {
            Ok(ret)
        } else {
            Err(Error::from(ret))
        }
    }

    /// Close the device.
    ///
    /// # Returns
    ///
    /// An `Ok` result if successful, otherwise an `Error`.
    pub fn close(&self) -> Result<()> {
        let ret = unsafe { rtlsdr_close(self.dev) };
        if ret == 0 {
            Ok(())
        } else {
            Err(Error::from(ret))
        }
    }

    /// Set the center frequency of the device.
    ///
    /// # Arguments
    ///
    /// * `rtl_freq_hz` - The frequency in Hz to set.
    /// * `tuner_freq_hz` - The tuner frequency in Hz to set.
    ///
    /// # Returns
    ///
    /// An `Ok` result if successful, otherwise an `Error`.
    pub fn set_xtal_freq(&self, rtl_freq_hz: u32, tuner_freq_hz: u32) -> Result<()> {
        let ret = unsafe { rtlsdr_set_xtal_freq(self.dev, rtl_freq_hz, tuner_freq_hz) };
        if ret == 0 {
            Ok(())
        } else {
            Err(Error::from(ret))
        }
    }

    /// Get the crystal frequency of the device.
    ///
    /// # Returns
    ///
    /// The device's crystal frequency as a tuple of `rtl_freq_hz` and `tuner_freq_hz
    pub fn get_xtal_freq(&self) -> Result<(u32, u32)> {
        let mut rtl_freq_hz: u32 = 0;
        let mut tuner_freq_hz: u32 = 0;
        let ret = unsafe { rtlsdr_get_xtal_freq(self.dev, &mut rtl_freq_hz, &mut tuner_freq_hz) };
        if ret == 0 {
            Ok((rtl_freq_hz, tuner_freq_hz))
        } else {
            Err(Error::from(ret))
        }
    }

    /// Get the USB strings of the device.
    ///
    /// # Returns
    ///
    /// The manufacturer, product, and serial strings of the device.
    pub fn get_usb_strings(&self) -> Result<(String, String, String)> {
        let mut m: [c_char; 256] = [0; 256];
        let mut p: [c_char; 256] = [0; 256];
        let mut s: [c_char; 256] = [0; 256];
        let ret = unsafe {
            rtlsdr_get_usb_strings(self.dev, m.as_mut_ptr(), p.as_mut_ptr(), s.as_mut_ptr())
        };
        if ret == 0 {
            let manufact = unsafe { CStr::from_ptr(m.as_ptr()) }
                .to_string_lossy()
                .into_owned();
            let product = unsafe { CStr::from_ptr(p.as_ptr()) }
                .to_string_lossy()
                .into_owned();
            let serial = unsafe { CStr::from_ptr(s.as_ptr()) }
                .to_string_lossy()
                .into_owned();
            Ok((manufact, product, serial))
        } else {
            Err(Error::from(ret))
        }
    }

    /// Write data to the EEPROM of the device.
    ///
    /// # Arguments
    ///
    /// * `data` - The data to write to the EEPROM.
    /// * `offset` - The offset to write the data to.
    ///
    /// # Returns
    ///
    /// An `Ok` result if successful, otherwise an `Error`.
    pub fn write_eeprom(&self, data: &[u8], offset: u8) -> Result<()> {
        let len = data.len() as u16;
        let ret = unsafe { rtlsdr_write_eeprom(self.dev, data.as_ptr() as *mut u8, offset, len) };
        if ret >= 0 {
            Ok(())
        } else {
            Err(Error::from(ret))
        }
    }

    /// Read data from the EEPROM of the device.
    ///
    /// # Arguments
    ///
    /// * `offset` - The offset to read the data from.
    /// * `len` - The length of the data to read.
    ///
    /// # Returns
    ///
    /// A vector of data read from the EEPROM.
    pub fn read_eeprom(&self, offset: u8, len: u16) -> Result<Vec<u8>> {
        let mut v = vec![0u8; len as usize];
        let ret = unsafe { rtlsdr_read_eeprom(self.dev, v.as_mut_ptr(), offset, len) };
        if ret >= 0 {
            Ok(v)
        } else {
            Err(Error::from(ret))
        }
    }

    /// Set the sample rate of the device.
    ///
    /// # Arguments
    ///
    /// * `rate_hz` - The sample rate in Hz to set.
    ///
    /// # Returns
    ///
    /// An `Ok` result if successful, otherwise an `Error`.
    pub fn set_center_freq(&self, freq_hz: u32) -> Result<()> {
        let ret = unsafe { rtlsdr_set_center_freq(self.dev, freq_hz) };
        if ret == 0 {
            Ok(())
        } else {
            Err(Error::from(ret))
        }
    }

    /// Get the center frequency of the device.
    ///
    /// # Returns
    ///
    /// The device's center frequency.
    pub fn get_center_freq(&self) -> Result<u32> {
        let freq = unsafe { rtlsdr_get_center_freq(self.dev) };
        if freq >= 0 {
            Ok(freq as u32)
        } else {
            Err(Error::from(freq))
        }
    }

    /// Set the frequency correction of the device.
    ///
    /// # Arguments
    ///
    /// * `ppm` - The frequency correction in parts per million (ppm) to set.
    ///
    /// # Returns
    ///
    /// An `Ok` result if successful, otherwise an `Error`.
    pub fn set_freq_correction(&self, ppm: i32) -> Result<()> {
        let ret = unsafe { rtlsdr_set_freq_correction(self.dev, ppm) };
        if ret == 0 {
            Ok(())
        } else {
            Err(Error::from(ret))
        }
    }

    /// Get the frequency correction of the device.
    ///
    /// # Returns
    ///
    /// The device's frequency correction in parts per million (ppm).
    pub fn get_freq_correction(&self) -> Result<i32> {
        let ppm = unsafe { rtlsdr_get_freq_correction(self.dev) };
        if ppm >= 0 {
            Ok(ppm)
        } else {
            Err(Error::from(ppm))
        }
    }

    /// Get the tuner type of the device.
    ///
    /// # Returns
    ///
    /// The device's tuner type as an `RTLSDRTuner` if successful, otherwise an `Error
    pub fn get_tuner_type(&self) -> Result<RTLSDRTuner> {
        let tuner_type = unsafe { rtlsdr_get_tuner_type(self.dev) };
        RTLSDRTuner::try_from(tuner_type)
    }

    /// Get the tuner gain mode of the device.
    ///
    /// # Returns
    ///
    /// The device's tuner gain mode as a Vec<i32> if successful, otherwise an `Error`.
    pub fn get_tuner_gains(&self) -> Result<Vec<i32>> {
        unsafe {
            let num_gains = rtlsdr_get_tuner_gains(self.dev, ptr::null_mut());
            if num_gains <= 0 {
                return Err(Error::from(num_gains));
            }
            let mut gains = vec![0; num_gains as usize];
            let ret = rtlsdr_get_tuner_gains(self.dev, gains.as_mut_ptr());
            if ret <= 0 {
                Err(Error::from(ret))
            } else {
                Ok(gains)
            }
        }
    }

    /// Set the tuner gain of the device.
    ///
    /// # Arguments
    ///
    /// * `gain` - The tuner gain to set.
    ///
    ///  # Returns
    ///
    /// An `Ok` result if successful, otherwise an `Error`.
    pub fn set_tuner_gain(&self, gain: i32) -> Result<()> {
        let ret = unsafe { rtlsdr_set_tuner_gain(self.dev, gain) };
        if ret == 0 {
            Ok(())
        } else {
            Err(Error::from(ret))
        }
    }

    /// Set the tuner bandwidth of the device.
    ///
    /// # Arguments
    ///
    /// * `bw_hz` - The tuner bandwidth in Hz to set.
    ///
    /// # Returns
    ///
    /// An `Ok` result if successful, otherwise an `Error`.
    pub fn set_tuner_bandwidth(&self, bw_hz: u32) -> Result<()> {
        let ret = unsafe { rtlsdr_set_tuner_bandwidth(self.dev, bw_hz) };
        if ret == 0 {
            Ok(())
        } else {
            Err(Error::from(ret))
        }
    }

    /// Get the tuner gain of the device.
    ///
    /// # Returns
    ///
    /// The device's tuner gain if successful, otherwise an `Error`.
    pub fn get_tuner_gain(&self) -> Result<i32> {
        let gain = unsafe { rtlsdr_get_tuner_gain(self.dev) };
        if gain >= 0 {
            Ok(gain)
        } else {
            Err(Error::from(gain))
        }
    }

    /// Set the tuner IF gain of the device.
    ///
    /// # Arguments
    ///
    /// * `stage` - The stage of the tuner IF gain to set.
    /// * `gain` - The tuner IF gain to set.
    ///
    /// # Returns
    ///
    /// An `Ok` result if successful, otherwise an `Error`.
    pub fn set_tuner_if_gain(&self, stage: i32, gain: i32) -> Result<()> {
        let ret = unsafe { rtlsdr_set_tuner_if_gain(self.dev, stage, gain) };
        if ret == 0 {
            Ok(())
        } else {
            Err(Error::from(ret))
        }
    }

    /// Set the tuner gain mode of the device.
    ///
    /// # Arguments
    ///
    /// * `manual_mode` - The tuner gain mode to set.
    ///
    /// # Returns
    ///
    /// An `Ok` result if successful, otherwise an `Error`.
    pub fn set_tuner_gain_mode(&self, manual_mode: bool) -> Result<()> {
        let ret = unsafe { rtlsdr_set_tuner_gain_mode(self.dev, manual_mode as c_int) };
        if ret == 0 {
            Ok(())
        } else {
            Err(Error::from(ret))
        }
    }

    /// Set the sample rate of the device.
    ///
    /// # Arguments
    ///
    /// * `rate_hz` - The sample rate in Hz to set.
    ///
    /// # Returns
    ///
    /// An `Ok` result if successful, otherwise an `Error`.
    pub fn set_sample_rate(&self, rate_hz: u32) -> Result<()> {
        let ret = unsafe { rtlsdr_set_sample_rate(self.dev, rate_hz) };
        if ret == 0 {
            Ok(())
        } else {
            Err(Error::from(ret))
        }
    }

    /// Get the sample rate of the device.
    ///
    /// # Returns
    ///
    /// The device's sample rate.
    pub fn get_sample_rate(&self) -> Result<u32> {
        let rate = unsafe { rtlsdr_get_sample_rate(self.dev) };
        if rate >= 0 {
            Ok(rate as u32)
        } else {
            Err(Error::from(rate))
        }
    }

    /// Set the test mode of the device.
    ///
    /// # Arguments
    ///
    /// * `on` - The test mode to set.
    ///
    /// # Returns
    ///
    /// An `Ok` result if successful, otherwise an `Error`.
    pub fn set_test_mode(&self, on: bool) -> Result<()> {
        let ret = unsafe { rtlsdr_set_test_mode(self.dev, on as c_int) };
        if ret == 0 {
            Ok(())
        } else {
            Err(Error::from(ret))
        }
    }

    /// Set the AGC mode of the device.
    ///
    /// # Arguments
    ///
    /// * `on` - The AGC mode to set.
    ///
    /// # Returns
    ///
    /// An `Ok` result if successful, otherwise an `Error`.
    pub fn set_agc_mode(&self, on: bool) -> Result<()> {
        let ret = unsafe { rtlsdr_set_agc_mode(self.dev, on as c_int) };
        if ret == 0 {
            Ok(())
        } else {
            Err(Error::from(ret))
        }
    }

    /// Set the direct sampling mode of the device.
    ///
    /// # Arguments
    ///
    /// * `on` - The direct sampling mode to set.
    ///
    /// # Returns
    ///
    /// An `Ok` result if successful, otherwise an `Error`.
    pub fn set_direct_sampling(&self, on: bool) -> Result<()> {
        let ret = unsafe { rtlsdr_set_direct_sampling(self.dev, on as c_int) };
        if ret == 0 {
            Ok(())
        } else {
            Err(Error::from(ret))
        }
    }

    /// Get the direct sampling state of the device.
    ///
    /// # Returns
    ///
    /// The device's direct sampling state.
    pub fn get_direct_sampling(&self) -> Result<bool> {
        let ret = unsafe { rtlsdr_get_direct_sampling(self.dev) };
        if ret >= 0 {
            Ok(ret != 0)
        } else {
            Err(Error::from(ret))
        }
    }

    /// Set the offset tuning mode of the device.
    ///
    /// # Arguments
    ///
    /// * `on` - The offset tuning mode to set.
    ///
    /// # Returns
    ///
    /// An `Ok` result if successful, otherwise an `Error`.
    pub fn set_offset_tuning(&self, on: bool) -> Result<()> {
        let ret = unsafe { rtlsdr_set_offset_tuning(self.dev, on as c_int) };
        if ret == 0 {
            Ok(())
        } else {
            Err(Error::from(ret))
        }
    }

    /// Get the offset tuning state of the device.
    ///
    /// # Returns
    ///
    /// The device's offset tuning state.
    pub fn get_offset_tuning(&self) -> Result<bool> {
        let ret = unsafe { rtlsdr_get_offset_tuning(self.dev) };
        if ret >= 0 {
            Ok(ret != 0)
        } else {
            Err(Error::from(ret))
        }
    }

    /// Reset the buffer of the device.
    ///
    /// # Returns
    ///
    /// An `Ok` result if successful, otherwise an `Error`.
    pub fn reset_buffer(&self) -> Result<()> {
        let ret = unsafe { rtlsdr_reset_buffer(self.dev) };
        if ret == 0 {
            Ok(())
        } else {
            Err(Error::from(ret))
        }
    }

    /// Read data from the device synchronously.
    ///
    /// # Arguments
    ///
    /// * `length` - The length of the data to read.
    ///
    /// # Returns
    ///
    /// A vector of data read from the device.
    pub fn read_sync(&self, length: usize) -> Result<Vec<u8>> {
        let mut buffer = vec![0u8; length];
        let ret = unsafe {
            rtlsdr_read_sync(
                self.dev,
                buffer.as_mut_ptr() as *mut c_void,
                length as c_int,
                ptr::null_mut(),
            )
        };
        if ret == 0 {
            Ok(buffer)
        } else {
            Err(Error::from(ret))
        }
    }

    /// Wait for asynchronous data to be read from the device.
    ///
    /// # Arguments
    ///
    /// * `callback` - The callback function to call when data is read.
    /// * `ctx` - The context to pass to the callback function.
    ///
    /// # Returns
    ///
    /// An `Ok` result if successful, otherwise an `Error`.
    pub fn wait_async(&self, callback: ReadAsyncCbT, ctx: *mut c_void) -> Result<()> {
        let ret = unsafe { rtlsdr_wait_async(self.dev, callback, ctx) };
        if ret == 0 {
            Ok(())
        } else {
            Err(Error::from(ret))
        }
    }

    /// Read data from the device asynchronously.
    ///
    /// # Arguments
    ///
    /// * `callback` - The callback function to call when data is read.
    /// * `ctx` - The context to pass to the callback function.
    ///
    /// # Returns
    ///
    /// An `Ok` result if successful, otherwise an `Error`.
    pub fn read_async(
        &self,
        callback: ReadAsyncCbT,
        ctx: *mut c_void,
        buf_num: u32,
        buf_len: u32,
    ) -> Result<()> {
        let ret = unsafe { rtlsdr_read_async(self.dev, callback, ctx, buf_num, buf_len) };
        if ret == 0 {
            Ok(())
        } else {
            Err(Error::from(ret))
        }
    }

    /// Cancel an asynchronous read operation.
    ///
    /// # Returns
    ///
    /// An `Ok` result if successful, otherwise an `Error`.
    pub fn cancel_async(&self) -> Result<()> {
        let ret = unsafe { rtlsdr_cancel_async(self.dev) };
        if ret == 0 {
            Ok(())
        } else {
            Err(Error::from(ret))
        }
    }

    /// Get the device's USB vendor, product ID, etc.
    ///
    /// # Returns
    ///
    /// The hardware information of the device as a `HwInfo` struct.
    pub fn get_hw_info(&self) -> Result<HwInfo> {
        let data = self.read_eeprom(0, EEPROM_SIZE as u16)?;
        if data.len() < STR_OFFSET_START {
            return Err(Error::NoValidEEPROMHeader);
        }

        if data[0] != 0x28 || data[1] != 0x32 {
            return Err(Error::NoValidEEPROMHeader);
        }

        let vendor_id = u16::from_le_bytes([data[2], data[3]]);
        let product_id = u16::from_le_bytes([data[4], data[5]]);
        let have_serial = data[6] == 0xA5;
        let remote_wakeup = (data[7] & 0x01) != 0;
        let enable_ir = (data[7] & 0x02) != 0;

        let (manufact, product, serial) = parse_string_descriptors(&data)?;

        Ok(HwInfo {
            vendor_id,
            product_id,
            manufact,
            product,
            serial,
            have_serial,
            enable_ir,
            remote_wakeup,
        })
    }

    /// Set the hardware information of the device.
    ///
    /// # Arguments
    ///
    /// * `info` - The hardware information to set as a `HwInfo` struct.
    ///
    /// # Returns
    ///
    /// An `Ok` result if successful, otherwise an `Error`.
    pub fn set_hw_info(&self, info: &HwInfo) -> Result<()> {
        let mut data = vec![0u8; EEPROM_SIZE];

        data[0] = 0x28;
        data[1] = 0x32;
        data[2..4].copy_from_slice(&info.vendor_id.to_le_bytes());
        data[4..6].copy_from_slice(&info.product_id.to_le_bytes());
        data[6] = if info.have_serial { 0xA5 } else { 0x00 };
        data[7] = 0x00;
        if info.remote_wakeup {
            data[7] |= 0x01;
        }
        if info.enable_ir {
            data[7] |= 0x02;
        }

        serialize_string_descriptors(&mut data, info)?;

        self.write_eeprom(&data, 0)
    }
}

impl Drop for Device {
    fn drop(&mut self) {
        match self.close() {
            Ok(_) => (),
            Err(e) => eprintln!("Error closing device: {}", e),
        }
    }
}
