// Hide the console window
#![windows_subsystem = "windows"]

use which::which;
use winreg::{enums::HKEY_CLASSES_ROOT, RegKey};
use winsafe::{co::MB, prelude::*, HWND};

fn main() {
    // discover javaw.exe
    let javaw = match which("javaw") {
        Ok(javaw) => javaw.display().to_string(),
        Err(_) => {
            // no javaw.exe found :'(
            HWND::GetDesktopWindow()
            .MessageBox("OpenJarFix could not find javaw.exe in your PATH environment variable.\nThis could mean that Java is not correctly installed or the PATH variable has not been updated.\n\nNo changes to your system have been made.",
            "OpenJarFix", MB::ICONWARNING).unwrap();
            std::process::exit(1);
        }
    };

    // set registry keys
    let hkcr: RegKey = RegKey::predef(HKEY_CLASSES_ROOT);

    let (key, _) = hkcr.create_subkey(".jar").unwrap();
    key.set_value("", &"jarfile").unwrap();

    let (key, _) = hkcr.create_subkey("jarfile").unwrap();
    key.set_value("", &"Executable Jar File").unwrap();
    let (key, _) = key.create_subkey("shell").unwrap();
    let (key, _) = key.create_subkey("open").unwrap();
    let (key, _) = key.create_subkey("command").unwrap();
    let _ = key.set_value("", &format!("\"{javaw}\" -jar \"%1\" %*"));

    // display success message
    HWND::GetDesktopWindow().MessageBox(&format!("The .jar (Java Archive) file extension has successfully been registered.\n\nUsed runtime:\n{javaw}"),
    "OpenJarFix", MB::ICONINFORMATION).unwrap();
}
