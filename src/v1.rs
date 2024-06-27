use bitflags::bitflags;
use bytemuck::{Pod, Zeroable};
use std::fmt::Debug;

use crate::interface::SteamDeckDeviceInterface;
use crate::DeviceIdentifier;
use crate::SteamDeckGamepadButton;

#[derive(Debug)]
pub struct SteamDeckInterface {
    device: hidraw::Device,
}

impl SteamDeckDeviceInterface for SteamDeckInterface {
    fn get_id() -> DeviceIdentifier
    where
        Self: Sized,
    {
        static ID: DeviceIdentifier = DeviceIdentifier {
            vendor_id: 0x28de,
            product_id: 0x1205,
            interface: 2,
        };

        ID
    }

    fn from_device(device: hidraw::Device) -> Self {
        Self { device }
    }

    fn get_device(&self) -> &hidraw::Device {
        &self.device
    }

    fn get_device_mut(&mut self) -> &mut hidraw::Device {
        &mut self.device
    }

    fn parse(&self, report: [u8; 64]) -> SteamDeckGamepadButton {
        let mut out = SteamDeckGamepadButton::empty();

        if report[2] == 9 {
            // handle input

            if let Some(flags) = ButtonFlags8::from_bits(report[8]) {
                map8(flags, &mut out);
            }
            if let Some(flags) = ButtonFlags9::from_bits(report[9]) {
                map9(flags, &mut out);
            }

            if let Some(flags) = ButtonFlags10::from_bits(report[10]) {
                map10(flags, &mut out);
            }

            if let Some(flags) = ButtonFlags11::from_bits(report[11]) {
                map11(flags, &mut out);
            }

            if let Some(flags) = ButtonFlags13::from_bits(report[13]) {
                map13(flags, &mut out);
            }

            if let Some(flags) = ButtonFlags14::from_bits(report[14]) {
                map14(flags, &mut out);
            }
        }

        out
    }
}

fn map8(btns: ButtonFlags8, out: &mut SteamDeckGamepadButton) {
    if btns.contains(ButtonFlags8::L1) {
        *out |= SteamDeckGamepadButton::L1;
    }

    if btns.contains(ButtonFlags8::L2) {
        *out |= SteamDeckGamepadButton::L2;
    }

    if btns.contains(ButtonFlags8::R1) {
        *out |= SteamDeckGamepadButton::R1;
    }

    if btns.contains(ButtonFlags8::R2) {
        *out |= SteamDeckGamepadButton::R2;
    }

    if btns.contains(ButtonFlags8::NORTH) {
        *out |= SteamDeckGamepadButton::NORTH;
    }

    if btns.contains(ButtonFlags8::SOUTH) {
        *out |= SteamDeckGamepadButton::SOUTH;
    }

    if btns.contains(ButtonFlags8::EAST) {
        *out |= SteamDeckGamepadButton::EAST;
    }

    if btns.contains(ButtonFlags8::WEST) {
        *out |= SteamDeckGamepadButton::WEST;
    }
}

fn map9(btns: ButtonFlags9, out: &mut SteamDeckGamepadButton) {
    if btns.contains(ButtonFlags9::DPAD_UP) {
        *out |= SteamDeckGamepadButton::DPAD_UP;
    }

    if btns.contains(ButtonFlags9::DPAD_DOWN) {
        *out |= SteamDeckGamepadButton::DPAD_DOWN;
    }

    if btns.contains(ButtonFlags9::DPAD_LEFT) {
        *out |= SteamDeckGamepadButton::DPAD_LEFT;
    }

    if btns.contains(ButtonFlags9::DPAD_RIGHT) {
        *out |= SteamDeckGamepadButton::DPAD_RIGHT;
    }

    if btns.contains(ButtonFlags9::STEAM) {
        *out |= SteamDeckGamepadButton::STEAM;
    }

    if btns.contains(ButtonFlags9::START) {
        *out |= SteamDeckGamepadButton::START;
    }

    if btns.contains(ButtonFlags9::SELECT) {
        *out |= SteamDeckGamepadButton::SELECT;
    }

    if btns.contains(ButtonFlags9::L5) {
        *out |= SteamDeckGamepadButton::L5;
    }
}

fn map10(btns: ButtonFlags10, out: &mut SteamDeckGamepadButton) {
    if btns.contains(ButtonFlags10::R5) {
        *out |= SteamDeckGamepadButton::R5;
    }

    if btns.contains(ButtonFlags10::LSTICK) {
        *out |= SteamDeckGamepadButton::LSTICK;
    }

    if btns.contains(ButtonFlags10::LPAD) {
        *out |= SteamDeckGamepadButton::LPAD;
    }

    if btns.contains(ButtonFlags10::LPAD_TOUCH) {
        *out |= SteamDeckGamepadButton::LPAD_TOUCH;
    }

    if btns.contains(ButtonFlags10::RPAD) {
        *out |= SteamDeckGamepadButton::RPAD;
    }

    if btns.contains(ButtonFlags10::RPAD_TOUCH) {
        *out |= SteamDeckGamepadButton::RPAD_TOUCH;
    }
}

fn map11(btns: ButtonFlags11, out: &mut SteamDeckGamepadButton) {
    if btns.contains(ButtonFlags11::RSTICK) {
        *out |= SteamDeckGamepadButton::RSTICK;
    }
}

fn map13(btns: ButtonFlags13, out: &mut SteamDeckGamepadButton) {
    if btns.contains(ButtonFlags13::L4) {
        *out |= SteamDeckGamepadButton::L4;
    }

    if btns.contains(ButtonFlags13::R4) {
        *out |= SteamDeckGamepadButton::R4;
    }

    if btns.contains(ButtonFlags13::LSTICK_TOUCH) {
        *out |= SteamDeckGamepadButton::LSTICK_TOUCH;
    }

    if btns.contains(ButtonFlags13::RSTICK_TOUCH) {
        *out |= SteamDeckGamepadButton::RSTICK_TOUCH;
    }
}

fn map14(btns: ButtonFlags14, out: &mut SteamDeckGamepadButton) {
    if btns.contains(ButtonFlags14::QAM) {
        *out |= SteamDeckGamepadButton::QAM;
    }
}

bitflags! {
    #[repr(transparent)]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash,  Pod, Zeroable)]
    struct ButtonFlags8: u8 {
        const R2 = 1;
        const L2 = 2;
        const R1 = 4;
        const L1 = 8;
        const NORTH = 16;
        const EAST = 32;
        const WEST = 64;
        const SOUTH = 128;

        // The source may set any bits
        const _ = !0;
    }
}

bitflags! {
    #[repr(transparent)]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash,  Pod, Zeroable)]
    struct ButtonFlags9: u8 {
        const DPAD_UP = 1;
        const DPAD_RIGHT = 2;
        const DPAD_LEFT = 4;
        const DPAD_DOWN = 8;
        const SELECT = 16;
        const STEAM = 32;
        const START = 64;
        const L5 = 128;

        // The source may set any bits
        const _ = !0;
    }
}

bitflags! {
    #[repr(transparent)]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash,  Pod, Zeroable)]
    struct ButtonFlags10: u8 {
        const R5 = 1;
        const LPAD = 2;
        const RPAD = 4;
        const LPAD_TOUCH = 8;
        const RPAD_TOUCH = 16;
        const LSTICK = 64;

        // The source may set any bits
        const _ = !0;
    }
}

bitflags! {
      #[repr(transparent)]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash,  Pod, Zeroable)]
    struct ButtonFlags11: u8 {
        const RSTICK = 4;

        // The source may set any bits
        const _ = !0;
    }
}

bitflags! {
    #[repr(transparent)]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash,  Pod, Zeroable)]
     struct ButtonFlags13: u8 {
        const L4 = 2;
        const R4 = 4;
        const LSTICK_TOUCH = 64;
        const RSTICK_TOUCH = 128;

        // The source may set any bits
        const _ = !0;
    }
}

bitflags! {
    #[repr(transparent)]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash,  Pod, Zeroable)]
    struct ButtonFlags14: u8 {
        const QAM = 4;

        // The source may set any bits
        const _ = !0;
    }
}
