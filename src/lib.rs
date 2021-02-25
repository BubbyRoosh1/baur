use colorful::*;

use git2::Repository;

use raur::{Raur, SearchBy};

use std::error::Error;
use std::fs;
use std::process::Command;

pub fn clean_cache() -> Result<(), Box<dyn Error>> {
    let mut cache_dir = dirs::cache_dir().unwrap();
    cache_dir.push("baur");
    fs::remove_dir_all(cache_dir)?;
    Ok(())
}

pub async fn install(name: &str) -> Result<(), Box<dyn Error>> {
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

            fs::remove_dir_all(&pkgdir).ok();
            Repository::clone(&url, &pkgdir)?;

            println!("Installing: {} Version: {}", package.name, package.version);
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

pub async fn query(search: &str) -> Result<(), Box<dyn Error>> {
    let raur = raur::Handle::new();
    let mut pkgs = raur.search_by(search, SearchBy::Name).await?;
    pkgs.sort_unstable_by(|a, b| a.name.cmp(&b.name));
    pkgs.iter().for_each(|p| {
        if let Some(desc) = &p.description {
            println!("{}: {}", p.name.clone().magenta(), desc);
        } else {
            println!("{}", p.name);
        }
    });
    Ok(())
}
