use rusb::{Device, GlobalContext};

const RAZER_VENDOR_ID: u16 = 0x1532;

const RAZER_PRODUCT_ID_WIRED: u16 = 0x00c0;
const RAZER_PRODUCT_ID_WIRELESS: u16 = 0x00c1;

pub fn get_device() -> rusb::Result<Option<Device<GlobalContext>>> {
    let devices = rusb::devices()?;

    for device in devices.iter() {
        let desc = match device.device_descriptor() {
            Ok(v) => v,
            Err(_) => continue,
        };

        if desc.vendor_id() != RAZER_VENDOR_ID {
            continue;
        }

        if !matches!(desc.product_id(), RAZER_PRODUCT_ID_WIRED | RAZER_PRODUCT_ID_WIRELESS) {
            continue;
        }

        return Ok(Some(device));
    }

    Ok(None)
}
