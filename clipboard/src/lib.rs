mod bindings {
    ::windows::include_bindings!();
}

use bindings::Windows::Win32::{
    DataExchange::*,
    SystemServices::*,
    WindowsAndMessaging::*,
};
use std::{cell::Cell, mem};

/// Inspiration: https://docs.microsoft.com/en-us/windows/win32/dataxchg/using-the-clipboard#copying-information-to-the-clipboard
pub struct Clipboard {
    is_open: Cell<bool>,
}

impl Default for Clipboard {
    fn default() -> Self {
        Self { is_open: Cell::new(false) }
    }
}

impl Clipboard {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn set_clipboard(&self, s: &str) -> Result<(), Error> {
        self.open_clipboard()?;

        let unicode = mem::ManuallyDrop::new(Lpwstr::from(s).into_inner());
        let handle = HANDLE(unicode.as_ptr() as _);

        let clipboard_set_failed = unsafe {
            SetClipboardData(
                CLIPBOARD_FORMATS::CF_UNICODETEXT.0,
                handle,
            )
        }.is_null();

        if clipboard_set_failed {
            mem::ManuallyDrop::into_inner(unicode);

            self.close_clipboard(clipboard_set_failed)?;
            Err(Error::FailedToSetClipboard)
        } else {
            self.close_clipboard(clipboard_set_failed)?;
            Ok(())
        }
    }

    /// https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-openclipboard
    fn open_clipboard(&self) -> Result<(), Error> {
        if self.is_open.get() {
            Err(Error::ClipboardAlreadyOpen)
        } else if unsafe { OpenClipboard(HWND::NULL) }.0 == 0 {
            // If an application calls OpenClipboard with hwnd set to NULL,
            // EmptyClipboard sets the clipboard owner to NULL; this causes
            // SetClipboardData to fail.
            Err(Error::FailedToOpenClipboard)
        } else {
            self.is_open.set(true);

            Ok(())
        }
    }

    /// https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-closeclipboard
    fn close_clipboard(&self, clipboard_set_failed: bool) -> Result<(), Error> {
        if !self.is_open.get() {
            Err(if clipboard_set_failed {
                Error::ClipboardAlreadyClosedAndFailedToSet
            } else {
                Error::ClipboardAlreadyClosed
            })
        } else if unsafe { CloseClipboard() }.0 == 0 {
            Err(if clipboard_set_failed {
                Error::FailedToSetAndCloseClipboard
            } else {
                Error::FailedToCloseClipboard
            })
        } else {
            self.is_open.set(false);

            Ok(())
        }
    }
}

struct Lpwstr(Vec<u16>);

impl<T: AsRef<str>> From<T> for Lpwstr {
    fn from(s: T) -> Self {
        Self(s.as_ref().encode_utf16().chain(Some(0)).collect())
    }
}

impl From<Lpwstr> for String {
    fn from(s: Lpwstr) -> Self {
        let ptr = s.0.as_ptr();

        if ptr.is_null() {
            Self::new()
        } else {
            let slice = unsafe {
                let mut len = 0;
                loop {
                    if *ptr.offset(len) == 0 {
                        break;
                    }

                    len += 1;
                }

                std::slice::from_raw_parts(ptr, len as _)
            };

            Self::from_utf16_lossy(slice)
        }
    }
}

impl Lpwstr {
    pub fn into_inner(self) -> Vec<u16> {
        self.0
    }
}

#[derive(Debug, PartialEq, thiserror::Error)]
pub enum Error {
    #[error("cannot open clipboard twice")]
    ClipboardAlreadyOpen,

    #[error("error opening clipboard")]
    FailedToOpenClipboard,

    #[error("error setting clipboard")]
    FailedToSetClipboard,

    #[error("cannot close clipboard twice")]
    ClipboardAlreadyClosed,

    #[error("error setting clipboard; cannot close clipboard twice")]
    ClipboardAlreadyClosedAndFailedToSet,

    #[error("error setting clipboard; error closing clipboard")]
    FailedToSetAndCloseClipboard,

    #[error("error closing clipboard")]
    FailedToCloseClipboard,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_lpwstr_round_trip() {
        let s = Lpwstr::from("abcdef");

        assert_eq!(
            String::from(s),
            "abcdef",
        );
    }

    #[test]
    fn test_clipboard_is_set() {
        let c = Clipboard::new();

        assert!(c.set_clipboard("abc").is_ok());

        // let clipboard = unsafe {
        //     open_clipboard();
        //     let data = GetClipboardData(CLIPBOARD_FORMATS::CF_UNICODETEXT.0).0;
        //     close_clipboard();

        //     data
        // };

        // assert_eq!(
        //     unsafe { Lpwstr::from_lpwstr(clipboard as *const _) },
        //     "abc"
        // );
    }

    #[test]
    fn test_clipboard_cannot_open_twice() {
        let c = Clipboard::new();

        c.open_clipboard().ok();
        let rv = c.open_clipboard();
        assert_eq!(rv.expect_err("rv is an error"), Error::ClipboardAlreadyOpen);

        c.close_clipboard(false).ok();
    }

    #[test]
    fn test_clipboard_cannot_close_twice() {
        let c = Clipboard::new();

        let rv = c.close_clipboard(false);
        assert_eq!(rv.expect_err("rv is an error"), Error::ClipboardAlreadyClosed);

        let rv = c.close_clipboard(true);
        assert_eq!(rv.expect_err("rv is an error"), Error::ClipboardAlreadyClosedAndFailedToSet);
    }
}
