use crate::error::{Error, Result};
use crate::hw_info::HwInfo;

pub const MAX_STR_SIZE: usize = 35;
pub const STR_OFFSET_START: usize = 0x09;
pub const EEPROM_SIZE: usize = 256;

/// Parse string descriptors from EEPROM data.
///
/// # Arguments
///
/// * `data` - EEPROM data.
///
/// # Returns
///
/// A tuple containing the manufacturer, product, and serial strings.
pub fn parse_string_descriptors(data: &[u8]) -> Result<(String, String, String)> {
    let mut pos = STR_OFFSET_START;
    let mut strings = Vec::new();

    for _ in 0..3 {
        if pos + 2 > data.len() {
            return Err(Error::StringDescriptorInvalid);
        }
        let length = data[pos] as usize;
        if length < 2 || pos + length > data.len() {
            return Err(Error::StringDescriptorInvalid);
        }
        if data[pos + 1] != 0x03 {
            return Err(Error::StringDescriptorInvalid);
        }
        let s = String::from_utf16(
            &data[pos + 2..pos + length]
                .chunks(2)
                .map(|pair| u16::from_le_bytes([pair[0], pair[1]]))
                .collect::<Vec<_>>(),
        )
        .map_err(|_| Error::StringDescriptorInvalid)?;
        strings.push(s);
        pos += length;
    }

    if strings.len() == 3 {
        Ok((strings[0].clone(), strings[1].clone(), strings[2].clone()))
    } else {
        Err(Error::Unknown)
    }
}

/// Serialize string descriptors to EEPROM data.
///
/// # Arguments
///
/// * `data` - EEPROM data.
/// * `info` - Hardware information.
pub fn serialize_string_descriptors(data: &mut Vec<u8>, info: &HwInfo) -> Result<()> {
    let mut pos = STR_OFFSET_START;
    let strings = [&info.manufact, &info.product, &info.serial];

    for s in strings {
        let utf16: Vec<u16> = s.encode_utf16().collect();
        let length = (utf16.len() * 2) + 2;
        if length > MAX_STR_SIZE * 2 + 2 {
            return Err(Error::StringValueTooLong);
        }
        if pos + length > data.len() {
            return Err(Error::StringDescriptorTooLong);
        }
        data[pos] = length as u8;
        data[pos + 1] = 0x03;
        for (i, &code_unit) in utf16.iter().enumerate() {
            let bytes = code_unit.to_le_bytes();
            data[pos + 2 + i * 2] = bytes[0];
            data[pos + 2 + i * 2 + 1] = bytes[1];
        }
        pos += length;
    }

    Ok(())
}
