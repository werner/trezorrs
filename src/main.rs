extern crate libusb;

const VENDOR_ID: u16 = 0x534c;
const PRODUCT_FIRMWARE: u8 = 0x0001;

struct TrezorContext {
    context: libusb::Context
}

impl TrezorContext {
    fn open(&mut self) -> libusb::Result<TrezorDevice> {
        let devices = self.context.devices()?;

        for device in devices.iter() {
            let device_desc = device.device_descriptor()?;

            if device_desc.vendor_id() == VENDOR_ID {
                let opened = device.open()?;
                return Ok(TrezorDevice { device_handle: opened });
            }
        }

        Err(libusb::Error::NoDevice)
    }
}

struct TrezorDevice<'a> {
    device_handle: libusb::DeviceHandle<'a>
}

fn main() {
    let context = libusb::Context::new().unwrap();

    let mut trezor_context = TrezorContext { context };

    let trezor_device = trezor_context.open();
    
    println!("Active Configuration: {:03}", trezor_device.unwrap().device_handle.active_configuration().unwrap());
}
