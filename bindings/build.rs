fn main() {
    windows::build!(
        Windows::Win32::DataExchange::{
            OpenClipboard,
            SetClipboardData,
            CloseClipboard,
        },
        Windows::Win32::SystemServices::CLIPBOARD_FORMATS,
        Windows::Win32::WindowsAndMessaging::HWND,
    );
}
