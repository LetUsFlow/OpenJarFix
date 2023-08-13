use std::env;

use embed_manifest::manifest::ExecutionLevel;
use embed_manifest::{embed_manifest, new_manifest};
use winres::WindowsResource;

fn main() {
    if env::var_os("CARGO_CFG_WINDOWS").is_some() {
        // Start program as administrator
        embed_manifest(
            new_manifest("openjarfix.manifest")
                .requested_execution_level(ExecutionLevel::RequireAdministrator),
        )
        .expect("unable to embed manifest file");

        WindowsResource::new()
            .set_icon("duke.ico")
            .compile()
            .expect("unable to set appicon");
    }

    println!("cargo:rerun-if-changed=build.rs");
}
