use std::process::Command;

pub fn export_brew() -> String {
    let brew_tap_list_output = match Command::new("brew").arg("tap").output() {
        Ok(x) => x,
        Err(e) => {
            panic!("{:?}", e)
        }
    };

    let brew_tap_string: String =
        String::from_utf8_lossy(&brew_tap_list_output.stdout)
            .split("\n")
            .filter(|it| { it.len() != 0 })
            .map(|it| { format!("brew tap {}", it) })
            .collect::<Vec<String>>()
            .join("\n");

    let brew_list_output = match Command::new("brew").arg("leaves").output() {
        Ok(x) => x,
        Err(e) => {
            panic!("{:?}", e)
        }
    };

    let brew_string = format!("brew install {}", String::from_utf8_lossy(&brew_list_output.stdout).replace("\n", " "));


    let brew_cask_list_output = match Command::new("brew")
        .arg("cask")
        .arg("list")
        .output() {
        Ok(x) => x,
        Err(e) => {
            panic!("{:?}", e)
        }
    };

    let brew_cask_string = format!("brew cask install {}", String::from_utf8_lossy(&brew_cask_list_output.stdout).replace("\n", " "));


    format!(r##"
# ====== BREW ======
# Install xcode
xcode-select --install

# Install homebrew
/bin/bash -c "$(curl -fsSL https://raw.githubusercontent.com/Homebrew/install/master/install.sh)"


# All your taps
{}

# All your formulae
{}

# All your casks
{}
"##,
             brew_tap_string,
             brew_string,
             brew_cask_string
    )
}

