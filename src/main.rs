// Copyright (C) 2021 BubbyRoosh
use rargsxd::*;
use raur::{Raur, SearchBy};
use git2::Repository;
use std::error::Error;
use std::fs;
use std::process::Command;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let mut parsed = ArgParser::new("baur");
    parsed.author("BubbyRoosh")
        .version("0.1.0")
        .copyright("Copyright (C) 2021 BubbyRoosh")
        .info("Bubby's AUR helper")
        .require_args(true)
        .args(vec!(
                Arg::new("install")
                    .short("i")
                    .help("Installs the given package(s)")
                    .option(""),
                Arg::new("query")
                    .short("q")
                    .help("Queries the AUR with the given name")
                    .option(""),
                Arg::new("clean")
                    .short("c")
                    .help("Cleans the cache")
                    .flag(false),
        ))
        .parse();

    if parsed.get_option("query").unwrap() != "" {query(&parsed.get_option("query").unwrap()).await?}
    if parsed.get_option("install").unwrap() != "" {
        for package in parsed.get_option("install").unwrap().split_whitespace() {
            install(package).await?;
        }
    }
    if parsed.get_flag("clean").unwrap() {clean_cache()?;println!("Cleaned cache")}
    Ok(())
}

fn clean_cache() -> Result<(), Box<dyn Error>> {
    let mut cache_dir = dirs::cache_dir().unwrap();
    cache_dir.push("baur");
    fs::remove_dir_all(cache_dir)?;
    Ok(())
}

async fn install(name: &str) -> Result<(), Box<dyn Error>> {
    let mut cache_dir = dirs::cache_dir().unwrap();
    cache_dir.push("baur");
    fs::create_dir_all(&cache_dir)?;

    let raur = raur::Handle::new();
    let pkgs = raur.search_by(name, SearchBy::Name).await?;
    for package in pkgs {
        if package.name == name {
            let mut url = String::from("https://aur.archlinux.org/");
            url += &package.name;
            url += ".git";
            let mut pkgdir = cache_dir.clone();
            pkgdir.push(&package.name);

            // TODO: Proper git pull stuff lol
            fs::remove_dir_all(&pkgdir).ok();
            Repository::clone(&url, &pkgdir)?;

            println!("Installing: {} Version: {}", package.name, package.version);
            // Moment
            Command::new("makepkg")
                .current_dir(pkgdir)
                .arg("-si")
                .spawn()?
                .wait()?;

            return Ok(())
        }
    }
    Ok(())
}

async fn query(search: &str) -> Result<(), Box<dyn Error>> {
    let raur = raur::Handle::new();
    let mut pkgs = raur.search_by(search, SearchBy::Name).await?;
    pkgs.sort_unstable_by(|a, b| a.name.cmp(&b.name));
    pkgs.iter().for_each(|p| {
        if let Some(desc) = &p.description {
            println!("{}: {}", p.name, desc);
        } else {
            println!("{}", p.name);
        }
    });
    Ok(())
}
