// Copyright (C) 2021 BubbyRoosh
use clap::{App, Arg};
use raur::{Raur, SearchBy};
use git2::Repository;
use std::error::Error;
use std::fs;
use std::process::Command;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let matches = App::new("baur")
        .version("0.1.0")
        .author("BubbyRoosh")
        .about("Bubby's AUR helper.. it's not bad I swear it's just minimalism lol\nCopyright (C) 2021 BubbyRoosh")
        .usage("baur [args] [packages]")
        .args(&[
            Arg::with_name("install")
                .help("Installs the given package(s)")
                .short("i")
                .takes_value(true),

            Arg::with_name("query")
                .help("Queries the AUR with the given name")
                .short("q")
                .takes_value(true),

            Arg::with_name("clean")
                .help("Cleans the cache")
                .short("c")
        ])
        .get_matches();

    if matches.is_present("query") {
        query(matches.value_of("query").unwrap()).await?;
    } else if matches.is_present("install") {
        for package in matches.value_of("install").unwrap().split_whitespace() {
            install(package).await?;
        }
    } else if matches.is_present("clean") {
        clean_cache()?;
    }

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
            Repository::clone(&url, &pkgdir)?;

            println!("Installing: {}", package.name);
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
    pkgs.iter().for_each(|p| println!("{}", p.name));
    Ok(())
}
