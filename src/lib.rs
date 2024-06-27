use std::{collections::HashMap, fmt::Debug, io};

use bitflags::bitflags;
use bytemuck::{Pod, Zeroable};
use hidraw::raw_info::BusType;
use interface::SteamDeckDeviceInterface;
use mio::{unix::SourceFd, Events, Interest, Poll, Token};
use rand::Rng;
use std::os::fd::AsRawFd;

mod v1;

#[derive(Debug)]
pub struct SteamDeckDevice(Box<dyn SteamDeckDeviceInterface>);

#[derive(Debug, Clone, Copy)]
struct DeviceIdentifier {
    vendor_id: i16,
    product_id: i16,
    interface: i16,
}

impl SteamDeckDevice {
    /// Returns Steam Deck controller device for v1 of the hidraw protocol. Returns an error if it does not exist.
    pub fn v1() -> Result<Self, std::io::Error> {
        v1::SteamDeckInterface::load_device()
            .map(Box::new)
            .map(|v| Self(v))
    }

    /// Returns Steam Deck controller device matching the best available version of the hidraw protocol. Returns an error if it does not exist.
    pub fn best() -> Result<Self, std::io::Error> {
        // Future proofing that likely isn't needed. Fun though.
        v1::SteamDeckInterface::load_device()
            .map(Box::new)
            .map(|v| Self(v))
    }

    pub fn event_loop(&mut self, tx: std::sync::mpsc::Sender<SteamDeckGamepadButton>) -> () {
        let event_token = Token(rand::thread_rng().gen());

        let mut poll = Poll::new().unwrap();
        let mut events = Events::with_capacity(128);

        poll.registry()
            .register(
                &mut SourceFd(&self.0.get_device().as_raw_fd()),
                event_token,
                Interest::READABLE,
            )
            .unwrap();

        loop {
            let poll_timeout = None;
            poll.poll(&mut events, poll_timeout).unwrap();

            for event in &events {
                match event.token() {
                    token if token == event_token => {
                        match self.0.get_device_mut().get_input_report_read::<[u8; 64]>(1) {
                            Ok(report) => {
                                let out = self.0.parse(report);
                                if !out.is_empty() {
                                    let res = tx.send(out);
                                    if let Err(_) = res {
                                        println!("Channel disconnected; exiting event loop.");
                                        return;
                                    }
                                }
                            }
                            Err(err) => println!("Error getting report: {err}"),
                        }
                    }
                    _ => (),
                }
            }
        }
    }
}

bitflags! {
    #[repr(transparent)]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash,  Pod, Zeroable)]
    pub struct SteamDeckGamepadButton: u32 {
        const R1 = 1;
        const R2 = 1 << 1;
        const L1 = 1 << 2;
        const L2 = 1 << 3;
        const NORTH = 1 << 4;
        const EAST = 1 << 5;
        const WEST = 1 << 6;
        const SOUTH = 1 << 7;
        const DPAD_UP = 1 << 8;
        const DPAD_RIGHT = 1 << 9;
        const DPAD_LEFT = 1 << 10;
        const DPAD_DOWN = 1 << 11;
        const START = 1 << 12;
        const SELECT = 1 << 13;
        const RSTICK = 1 << 14;
        const LSTICK = 1 << 15;
        const STEAM = 1 << 16;
        const QAM = 1 << 17;
        const R4 = 1 << 18;
        const R5 = 1 << 19;
        const L4 = 1 << 20;
        const L5 = 1 << 21;
        const RSTICK_TOUCH = 1 << 22;
        const LSTICK_TOUCH = 1 << 23;
        const LPAD = 1 << 24;
        const RPAD = 1 << 25;
        const LPAD_TOUCH = 1 << 26;
        const RPAD_TOUCH = 1 << 27;
    }
}

impl SteamDeckGamepadButton {
    pub fn display_name_to_value() -> HashMap<String, SteamDeckGamepadButton> {
        let map = HashMap::from_iter(
            [
                ("R1", SteamDeckGamepadButton::R1),
                ("R2", SteamDeckGamepadButton::R2),
                ("R4", SteamDeckGamepadButton::R4),
                ("R5", SteamDeckGamepadButton::R5),
                ("L1", SteamDeckGamepadButton::L1),
                ("L2", SteamDeckGamepadButton::L2),
                ("L4", SteamDeckGamepadButton::L4),
                ("L5", SteamDeckGamepadButton::L5),
                ("North (Y)", SteamDeckGamepadButton::NORTH),
                ("South (A)", SteamDeckGamepadButton::SOUTH),
                ("East (B)", SteamDeckGamepadButton::EAST),
                ("West (X)", SteamDeckGamepadButton::WEST),
                ("DPad-Up", SteamDeckGamepadButton::DPAD_UP),
                ("DPad-Down", SteamDeckGamepadButton::DPAD_DOWN),
                ("DPad-Left", SteamDeckGamepadButton::DPAD_LEFT),
                ("DPad-Right", SteamDeckGamepadButton::DPAD_RIGHT),
                ("Start", SteamDeckGamepadButton::START),
                ("Select", SteamDeckGamepadButton::SELECT),
                ("RStick", SteamDeckGamepadButton::RSTICK),
                ("LStick", SteamDeckGamepadButton::LSTICK),
                ("Steam", SteamDeckGamepadButton::STEAM),
                ("QAM", SteamDeckGamepadButton::QAM),
                ("RStick (Touch)", SteamDeckGamepadButton::RSTICK_TOUCH),
                ("LStick (Touch)", SteamDeckGamepadButton::LSTICK_TOUCH),
                ("RPad", SteamDeckGamepadButton::RPAD),
                ("LPad", SteamDeckGamepadButton::LPAD),
                ("RPad (Touch)", SteamDeckGamepadButton::RPAD_TOUCH),
                ("LPad (Touch)", SteamDeckGamepadButton::LPAD_TOUCH),
            ]
            .map(|(k, v)| (k.to_string(), v)),
        );

        debug_assert_eq!(map.len(), SteamDeckGamepadButton::all().into_iter().count());

        map
    }

    pub fn value_to_display_name() -> HashMap<SteamDeckGamepadButton, String> {
        Self::display_name_to_value()
            .into_iter()
            .map(|(k, v)| (v, k))
            .collect()
    }
}

pub(crate) mod interface {
    use super::*;

    pub trait SteamDeckDeviceInterface: Debug + Send {
        fn get_id() -> DeviceIdentifier
        where
            Self: Sized;

        fn load_device() -> Result<Self, io::Error>
        where
            Self: Sized,
        {
            let dev = "/dev/";

            // TODO::error/warning logging

            for file in std::fs::read_dir(&dev)? {
                if let Ok(file) = file {
                    if file.path().exists()
                        && file.file_name().to_string_lossy().starts_with("hidraw")
                    {
                        if let Ok(mut device) = hidraw::Device::open(file.path()) {
                            let info = device.get_raw_info();
                            if let Ok(info) = info {
                                if info.product() == Self::get_id().product_id
                                    && info.vendor() == Self::get_id().vendor_id
                                    && info.bus_type() == BusType::Usb
                                {
                                    if let Ok(addr) = device.get_physical_address() {
                                        if addr.ends_with(&format!(
                                            "input{}",
                                            Self::get_id().interface
                                        )) {
                                            return Ok(Self::from_device(device));
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }

            Err(io::Error::other(
                "failed to find hidraw file matching device",
            ))
        }

        fn from_device(device: hidraw::Device) -> Self
        where
            Self: Sized;

        fn get_device(&self) -> &hidraw::Device;
        fn get_device_mut(&mut self) -> &mut hidraw::Device;

        fn parse(&self, report: [u8; 64]) -> SteamDeckGamepadButton;
    }
}
