// Hide the console window
#![windows_subsystem = "windows"]

use std::io;
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
            .MessageBox("OpenJarFix could not find javaw.exe in your PATH environment variable.\n\
            This could mean that Java is not correctly installed or the PATH variable has not been updated.\n\n\
            No changes to your system have been made.", "OpenJarFix", MB::ICONWARNING).unwrap();
            std::process::exit(1);
        }
    };

    // set registry keys
    match add_jar_registry_keys(&javaw) {
        Ok(_) => {
            // display success message
            HWND::GetDesktopWindow()
            .MessageBox(&format!("The .jar (Java Archive) file extension has successfully been registered.\n\n\
            Used runtime:\n{javaw}"), "OpenJarFix", MB::ICONINFORMATION).unwrap();
        }
        Err(_) => {
            // display error message
            HWND::GetDesktopWindow()
            .MessageBox("An error occured while setting the necessary registry keys.\n\
            The .jar file extension might not work.", "OpenJarFix", MB::ICONERROR).unwrap();
        }
    };
}

fn add_jar_registry_keys(javaw: &str) -> io::Result<()> {
    let hkcr: RegKey = RegKey::predef(HKEY_CLASSES_ROOT);
    let (key, _) = hkcr.create_subkey(".jar")?;
    key.set_value("", &"jarfile")?;

    let (key, _) = hkcr.create_subkey("jarfile")?;
    key.set_value("", &"Executable Jar File")?;
    let (key, _) = key.create_subkey("shell")?;
    let (key, _) = key.create_subkey("open")?;
    let (key, _) = key.create_subkey("command")?;
    let _ = key.set_value("", &format!("\"{javaw}\" -jar \"%1\" %*"));
    Ok(())
}
