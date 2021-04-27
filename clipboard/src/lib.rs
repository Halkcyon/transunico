mod bindings {
    ::windows::include_bindings!();
}

mod private {
    pub trait Sealed {}

    impl<T: AsRef<str>> Sealed for T {}
}

use bindings::Windows::Win32::{
    DataExchange::*,
    SystemServices::{CLIPBOARD_FORMATS, HANDLE},
    WindowsAndMessaging::HWND,
};
use std::{error, fmt};

/// Inspiration: https://docs.microsoft.com/en-us/windows/win32/dataxchg/using-the-clipboard#copying-information-to-the-clipboard
pub fn set_clipboard(s: &str) -> Result<(), ErrorKind> {
    if !open_clipboard() {
        return Err(ErrorKind::OpenClipboard);
    }

    let unicode = s.to_lpwstr();
    let handle = HANDLE(unicode.as_ptr() as _);
    // If the function succeeds, the return value is the handle to the data.
    // If the function fails, the return value is NULL.
    let failed_to_set_clipboard = unsafe {
        // If SetClipboardData succeeds, the system owns the object identified
        // by the hMem parameter. The application may not write to or free the
        // data once ownership has been transferred to the system, but it can
        // lock and read from the data until the CloseClipboard function is
        // called. (The memory must be unlocked before the Clipboard is
        // closed.) If the hMem parameter identifies a memory object, the
        // object must have been allocated using the function with the
        // GMEM_MOVEABLE flag.
        // https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-setclipboarddata
        SetClipboardData(
            // Unicode text format. Each line ends with a carriage
            // return/linefeed (CR-LF) combination. A null character signals
            // the end of the data.
            // https://docs.microsoft.com/en-us/windows/win32/dataxchg/standard-clipboard-formats#constants
            CLIPBOARD_FORMATS::CF_UNICODETEXT.0,
            handle,
        )
    }.is_null();

    if !close_clipboard() {
        if failed_to_set_clipboard {
            Err(ErrorKind::SetAndCloseClipboard)
        } else {
            Err(ErrorKind::CloseClipboard)
        }
    } else if failed_to_set_clipboard {
        Err(ErrorKind::SetClipboardData)
    } else {
        Ok(())
    }
}

fn open_clipboard() -> bool {
    // If the function succeeds, the return value is nonzero.
    // If the function fails, the return value is zero.
    unsafe {
        // Opens the clipboard for examination and prevents other applications from
        // modifying the clipboard content.
        // OpenClipboard fails if another window has the clipboard open.
        // An application should call the CloseClipboard function after every
        // successful call to OpenClipboard.
        // The window identified by the hWndNewOwner parameter does not become
        // the clipboard owner unless the EmptyClipboard function is called.
        // If an application calls OpenClipboard with hwnd set to NULL,
        // EmptyClipboard sets the clipboard owner to NULL; this causes
        // SetClipboardData to fail.
        // https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-openclipboard
        OpenClipboard(
            // A handle to the window to be associated with the open clipboard.
            // If this parameter is NULL, the open clipboard is associated with
            // the current task.
            HWND::NULL
        )
    }.0 != 0
}

fn close_clipboard() -> bool {
    // If the function succeeds, the return value is nonzero.
    // If the function fails, the return value is zero.
    unsafe {
        // When the window has finished examining or changing the clipboard,
        // close the clipboard by calling CloseClipboard. This enables other
        // windows to access the clipboard.
        // Do not place an object on the clipboard after calling CloseClipboard.
        // https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-closeclipboard
        CloseClipboard()
    }.0 != 0
}

pub trait ToLpwstr: private::Sealed {
    fn to_lpwstr(&self) -> Vec<u16>;
}

impl<T> ToLpwstr for T
where
    T: AsRef<str>
{
    fn to_lpwstr(&self) -> Vec<u16> {
        self.as_ref()
            .encode_utf16()
            .chain(std::iter::once(0))
            .collect()
    }
}

pub struct Lpwstr;

impl Lpwstr {
    /// # Safety
    /// You must ensure your slice has a null-terminating byte.
    pub unsafe fn from_lpwstr(ptr: *const u16) -> String {
        use std::slice::from_raw_parts;

        if ptr.is_null() {
            return String::new();
        }

        let slice = {
            let mut len = 0;
            loop {
                if *ptr.offset(len) == 0 {
                    break;
                }
                len += 1;
            }

            from_raw_parts(ptr, len as _)
        };

        String::from_utf16(slice).unwrap()
    }
}

#[derive(Debug)]
pub enum ErrorKind {
    OpenClipboard,
    SetClipboardData,
    SetAndCloseClipboard,
    CloseClipboard,
}

impl fmt::Display for ErrorKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl error::Error for ErrorKind {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_round_trip() {
        let string = "abcdef";
        let mut lpwstr = string.to_lpwstr();

        assert_eq!(
            unsafe { Lpwstr::from_lpwstr(lpwstr.as_mut_ptr()) },
            string
        );
    }

    #[test]
    fn test_clipboard_is_set() {
        assert!(set_clipboard("abc").is_ok());

        let clipboard = unsafe {
            open_clipboard();
            let data = GetClipboardData(CLIPBOARD_FORMATS::CF_UNICODETEXT.0).0;
            close_clipboard();

            data
        };

        assert_eq!(
            unsafe { Lpwstr::from_lpwstr(clipboard as *const _) },
            "abc"
        );
    }
}
