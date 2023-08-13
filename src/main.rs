// Hide the console window
#![windows_subsystem = "windows"]

extern crate winapi;

use std::ffi::CString;
use which::which;
use winapi::um::winuser::{MessageBoxA, MB_ICONINFORMATION, MB_ICONWARNING};
use winreg::{enums::*, RegKey};

fn main() {
    let lp_caption = CString::new("OpenJarFix").unwrap();

    let javaw = match which("javaw") {
        Ok(javaw) => javaw.display().to_string(),
        Err(_) => {
            let lp_text = CString::new("OpenJarFix could not find javaw.exe in your PATH environment variable.\nThis could mean that Java is not correctly installed or the PATH variable has not been updated.\n\nNo changes to your system have been made.").unwrap();
            unsafe {
                MessageBoxA(
                    std::ptr::null_mut(),
                    lp_text.as_ptr(),
                    lp_caption.as_ptr(),
                    MB_ICONWARNING,
                );
            }
            std::process::exit(1);
        }
    };

    let command = format!("\"{javaw}\" -jar \"%1\" %*");

    let hkcr: RegKey = RegKey::predef(HKEY_CLASSES_ROOT);

    let (key, _) = hkcr.create_subkey(".jar").unwrap();
    key.set_value("", &"jarfile").unwrap();

    let (key, _) = hkcr.create_subkey("jarfile").unwrap();
    key.set_value("", &"Executable Jar File").unwrap();
    let (key, _) = key.create_subkey("shell").unwrap();
    let (key, _) = key.create_subkey("open").unwrap();
    let (key, _) = key.create_subkey("command").unwrap();
    let _ = key.set_value("", &command);

    // https://github.com/mxre/winres/blob/master/example/src/main.rs
    let lp_text = CString::new(format!("The .jar (Java Archive) file extension has successfully been registered.\n\nUsed runtime:\n{javaw}")).unwrap();
    unsafe {
        MessageBoxA(
            std::ptr::null_mut(),
            lp_text.as_ptr(),
            lp_caption.as_ptr(),
            MB_ICONINFORMATION,
        );
    }
}
