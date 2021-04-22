fn main() {
    println!("cargo:rerun-if-changed=build.rs");
    windows::build!(
        Windows::Win32::DataExchange::{
            OpenClipboard,
            EmptyClipboard,
            SetClipboardData,
            GetClipboardData,
            CloseClipboard,
        },
        Windows::Win32::SystemServices::CLIPBOARD_FORMATS,
        Windows::Win32::WindowsAndMessaging::HWND,
    );
}
