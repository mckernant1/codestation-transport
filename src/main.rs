mod brew;
mod npm;
mod zsh;
mod p10k;

#[macro_use]
extern crate clap;

use clap::{App, AppSettings};

fn main() {
    let yaml = load_yaml!("cli.yml");
    let app = App::from_yaml(yaml)
        .setting(AppSettings::ArgRequiredElseHelp);

    let matches = app.get_matches();

    let packages = matches.values_of("PACKAGES").unwrap()
        .map(|it| { it.to_ascii_lowercase() })
        .collect::<Vec<String>>();
    let mut string_vec: Vec<String> = vec![];
    for package in packages {
        let package_string = match package.as_str() {
            "brew" => brew::export_brew(),
            "npm" => npm::export_npm(),
            "zsh" => zsh::export_zsh(),
            "p10k" => p10k::export_p10k(),
            _ => {
                panic!("{} does not match any available options", package)
            }
        };
        string_vec.push(package_string);
    }

    println!("{}", string_vec.join("\n\n"))

}
