use std::process::Command;
use regex::Regex;

pub fn export_npm() -> String {
    let npm_global_list = match Command::new("npm")
        .arg("list")
        .arg("-g")
        .arg("--depth=0")
        .output() {
        Ok(x) => x,
        Err(e) => {
            panic!("{:?}", e)
        }
    };

    let get_npm_dep_regex = Regex::new(r"([@]?[\w+]+[/]?[\w+]*)@")
        .unwrap();
    let npm_string = String::from_utf8_lossy(&npm_global_list.stdout)
        .split("\n")
        .filter_map(|it| {
            get_npm_dep_regex.captures(it)?.get(1)
        }).map(|it| it.as_str())
        .collect::<Vec<&str>>()
        .join(" ");

    return format!(r##"
# Install Global NPM dependencies
npm i -g {}
    "##, npm_string);
}
