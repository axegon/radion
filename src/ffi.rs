use std::os::raw::{c_char, c_int, c_uchar, c_void};
pub enum RTLSDRDevT {}

pub type ReadAsyncCbT = Option<unsafe extern "C" fn(buf: *mut c_uchar, len: u32, ctx: *mut c_void)>;

#[link(name = "rtlsdr")]
#[allow(improper_ctypes)]
extern "C" {
    pub fn rtlsdr_get_device_count() -> u32;
    pub fn rtlsdr_get_device_name(index: u32) -> *const c_char;
    pub fn rtlsdr_get_device_usb_strings(
        index: *mut RTLSDRDevT,
        manufact: *mut c_char,
        product: *mut c_char,
        serial: *mut c_char,
    ) -> c_int;
    pub fn rtlsdr_get_index_by_serial(serial: *const c_char) -> c_int;

    pub fn rtlsdr_open(dev: *mut *mut RTLSDRDevT, index: u32) -> c_int;
    pub fn rtlsdr_close(dev: *mut RTLSDRDevT) -> c_int;
    pub fn rtlsdr_set_xtal_freq(dev: *mut RTLSDRDevT, rtl_freq: u32, tuner_freq: u32) -> c_int;
    pub fn rtlsdr_get_xtal_freq(
        dev: *mut RTLSDRDevT,
        rtl_freq: *mut u32,
        tuner_freq: *mut u32,
    ) -> c_int;
    pub fn rtlsdr_get_usb_strings(
        dev: *mut RTLSDRDevT,
        manufact: *mut c_char,
        product: *mut c_char,
        serial: *mut c_char,
    ) -> c_int;
    pub fn rtlsdr_write_eeprom(dev: *mut RTLSDRDevT, data: *mut u8, offset: u8, len: u16) -> c_int;
    pub fn rtlsdr_read_eeprom(dev: *mut RTLSDRDevT, data: *mut u8, offset: u8, len: u16) -> c_int;
    pub fn rtlsdr_set_center_freq(dev: *mut RTLSDRDevT, freq: u32) -> c_int;
    pub fn rtlsdr_get_center_freq(dev: *mut RTLSDRDevT) -> c_int;
    pub fn rtlsdr_set_freq_correction(dev: *mut RTLSDRDevT, ppm: c_int) -> c_int;
    pub fn rtlsdr_get_freq_correction(dev: *mut RTLSDRDevT) -> c_int;
    pub fn rtlsdr_get_tuner_type(dev: *mut RTLSDRDevT) -> c_int;
    pub fn rtlsdr_get_tuner_gains(dev: *mut RTLSDRDevT, gains: *mut c_int) -> c_int;
    pub fn rtlsdr_set_tuner_gain(dev: *mut RTLSDRDevT, gain: c_int) -> c_int;
    pub fn rtlsdr_set_tuner_bandwidth(dev: *mut RTLSDRDevT, bw: u32) -> c_int;
    pub fn rtlsdr_get_tuner_gain(dev: *mut RTLSDRDevT) -> c_int;
    pub fn rtlsdr_set_tuner_if_gain(dev: *mut RTLSDRDevT, stage: c_int, gain: c_int) -> c_int;
    pub fn rtlsdr_set_tuner_gain_mode(dev: *mut RTLSDRDevT, manual: c_int) -> c_int;
    pub fn rtlsdr_set_sample_rate(dev: *mut RTLSDRDevT, rate: u32) -> c_int;
    pub fn rtlsdr_get_sample_rate(dev: *mut RTLSDRDevT) -> c_int;
    pub fn rtlsdr_set_test_mode(dev: *mut RTLSDRDevT, on: c_int) -> c_int;
    pub fn rtlsdr_set_agc_mode(dev: *mut RTLSDRDevT, on: c_int) -> c_int;
    pub fn rtlsdr_set_direct_sampling(dev: *mut RTLSDRDevT, on: c_int) -> c_int;
    pub fn rtlsdr_get_direct_sampling(dev: *mut RTLSDRDevT) -> c_int;
    pub fn rtlsdr_set_offset_tuning(dev: *mut RTLSDRDevT, on: c_int) -> c_int;
    pub fn rtlsdr_get_offset_tuning(dev: *mut RTLSDRDevT) -> c_int;
    pub fn rtlsdr_reset_buffer(dev: *mut RTLSDRDevT) -> c_int;
    pub fn rtlsdr_read_sync(
        dev: *mut RTLSDRDevT,
        buf: *mut c_void,
        len: c_int,
        n_read: *mut c_int,
    ) -> c_int;
    pub fn rtlsdr_wait_async(dev: *mut RTLSDRDevT, cb: ReadAsyncCbT, ctx: *mut c_void) -> c_int;
    pub fn rtlsdr_read_async(
        dev: *mut RTLSDRDevT,
        cb: ReadAsyncCbT,
        ctx: *mut c_void,
        buf_num: u32,
        buf_len: u32,
    ) -> c_int;
    pub fn rtlsdr_cancel_async(dev: *mut RTLSDRDevT) -> c_int;
}
