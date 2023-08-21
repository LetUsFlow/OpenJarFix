// Hide the console window
#![windows_subsystem = "windows"]

use std::io;
use which::which;
use winreg::{enums::HKEY_CLASSES_ROOT, RegKey};
use windows::{
    core::PCSTR,
    Win32::UI::WindowsAndMessaging::{
        MessageBoxA, MB_ICONERROR, MB_ICONINFORMATION, MB_ICONWARNING, MESSAGEBOX_STYLE,
    },
};

fn main() {
    // discover javaw.exe
    let javaw = match which("javaw") {
        Ok(javaw) => javaw.display().to_string(),
        Err(_) => {
            // no javaw.exe found :'(
            display_message_box("OpenJarFix could not find javaw.exe in your PATH environment variable.\n\
            This could mean that Java is not correctly installed or the PATH variable has not been updated.\n\n\
            No changes to your system have been made.", MB_ICONWARNING);
            // terminate the program
            return;
        }
    };

    // set registry keys
    match add_jar_registry_keys(&javaw) {
        Ok(_) => {
            // display success message
            display_message_box(
                &format!(
                    "The .jar (Java Archive) file extension has successfully been registered.\n\n\
                    Used runtime:\n{javaw}\n\n\
                    You should be able to execute .jar files with double-click now."
                ),
                MB_ICONINFORMATION,
            );
        }
        Err(_) => {
            // display error message
            display_message_box(
                "An error occured while setting the necessary registry keys.\n\
                The .jar file extension will not work.",
                MB_ICONERROR,
            );
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

fn display_message_box(text: &str, messagebox_style: MESSAGEBOX_STYLE) {
    unsafe {
        MessageBoxA(
            None,
            PCSTR::from_raw(format!("{text}\u{0}").as_ptr()),
            PCSTR::from_raw("OpenJarFix\u{0}".as_ptr()),
            messagebox_style,
        );
    }
}
