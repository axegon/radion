use crate::error::Error;
use std::convert::TryFrom;
use std::os::raw::c_int;

#[derive(Copy, Clone, Debug)]
#[repr(i32)]
pub enum RTLSDRTuner {
    Unknown = 0,
    E4000 = 1,
    FC0012 = 2,
    FC0013 = 3,
    FC2580 = 4,
    R820T = 5,
    R828D = 6,
}

pub enum SamplingMode {
    None = 0,
    IADC = 1,
    QADC = 2,
    Error = 3,
}

impl TryFrom<c_int> for RTLSDRTuner {
    type Error = Error;

    fn try_from(value: c_int) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(RTLSDRTuner::Unknown),
            1 => Ok(RTLSDRTuner::E4000),
            2 => Ok(RTLSDRTuner::FC0012),
            3 => Ok(RTLSDRTuner::FC0013),
            4 => Ok(RTLSDRTuner::FC2580),
            5 => Ok(RTLSDRTuner::R820T),
            6 => Ok(RTLSDRTuner::R828D),
            _ => Err(Error::Unknown),
        }
    }
}
