extern crate libusb;

const VENDOR_ID: u16 = 0x534c;
const INTERFACE: u8 = 0x000;
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
                return Ok(TrezorDevice { device_handle: opened, device });
            }
        }

        Err(libusb::Error::NoDevice)
    }
}

struct TrezorDevice<'a> {
    device_handle: libusb::DeviceHandle<'a>,
    device: libusb::Device<'a>
}

fn main() {
    let context = libusb::Context::new().unwrap();

    let mut trezor_context = TrezorContext { context };

    let result_trezor_device = trezor_context.open();
    let trezor_device = result_trezor_device.unwrap();
    let active_config_descriptor = trezor_device.device.active_config_descriptor().expect("No Config Descriptor");
    let mut device_handle = trezor_device.device_handle;

    for interface in active_config_descriptor.interfaces() {
        let result = device_handle.claim_interface(interface.number());
        if let Ok(claimed) = result {
            println!("claimed");
        }
        println!("Interface number: {:03}", interface.number());
        device_handle.release_interface(interface.number());
    }
    
    //println!("Active Configuration: {:03}", trezor_device.unwrap().device_handle.active_configuration().unwrap());
}
