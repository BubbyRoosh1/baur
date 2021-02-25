// Copyright (C) 2021 BubbyRoosh


use rargsxd::*;

use std::error::Error;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let mut parsed = ArgParser::new("baur");
    parsed.author("BubbyRoosh")
        .version("0.1.0")
        .copyright("Copyright (C) 2021 BubbyRoosh")
        .info("Bubby's AUR helper")
        .usage("baur [-i/-q/-c] <package(s)>")
        .require_args(true)
        .args(vec!(
                Arg::new("install")
                    .short('i')
                    .help("Installs the package(s)")
                    .flag(false),

                Arg::new("query")
                    .short('q')
                    .help("Queries the AUR with the given name(s)")
                    .flag(false),

                Arg::new("clean")
                    .short('c')
                    .help("Cleans the cache")
                    .flag(false),
        ))
        .parse();

    if parsed.get_flag("query").unwrap() {
        for package in &parsed.extra {
            baur::query(&package).await?;
        }
    }

    if parsed.get_flag("install").unwrap() {
        for package in &parsed.extra {
            baur::install(&package).await?;
        }
    }

    if parsed.get_flag("clean").unwrap() {
        baur::clean_cache()?;println!("Cleaned cache")
    }
    Ok(())
}
