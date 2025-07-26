use std::env;

use embed_manifest::manifest::ExecutionLevel;
use embed_manifest::{embed_manifest, new_manifest};

fn main() {
    if env::var_os("CARGO_CFG_WINDOWS").is_some() {
        // Start program as administrator
        embed_manifest(
            new_manifest("openjarfix.manifest")
                .requested_execution_level(ExecutionLevel::RequireAdministrator),
        )
        .expect("unable to embed manifest file");

        // Embed icon
        let _ = embed_resource::compile("app.rc", embed_resource::NONE);
    }

    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-changed=app.rc");
}
