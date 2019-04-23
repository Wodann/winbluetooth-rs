#![cfg(windows)]
#![allow(bad_style)]

extern crate winapi;

#[macro_use]
mod macros;

pub mod shared {
    pub mod bthdef;
    pub mod bthioctl;
    pub mod bthsdpdef;
}

pub mod um {
    pub mod bluetoothapis;
    pub mod bluetoothleapis;
    pub mod bthledef;
    pub mod ws2bth;
}
