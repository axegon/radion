#[derive(Debug)]
pub struct HwInfo {
    pub vendor_id: u16,
    pub product_id: u16,
    pub manufact: String,
    pub product: String,
    pub serial: String,
    pub have_serial: bool,
    pub enable_ir: bool,
    pub remote_wakeup: bool,
}
