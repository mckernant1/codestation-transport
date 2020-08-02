use std::process::Command;

pub fn export_zsh() -> String {
    let zshrc_output = match Command::new("cat")
        .arg(format!("{}/.zshrc", env!("HOME")))
        .output() {
        Ok(x) => x,
        Err(e) => {
            panic!("{:?}", e)
        }
    };
    let zshrc_string =
        String::from_utf8_lossy(&zshrc_output.stdout)
            .replace("'", "\"");

    return format!(r##"
# Install oh-my-zsh
sh -c "$(curl -fsSL https://raw.github.com/ohmyzsh/ohmyzsh/master/tools/install.sh)"

# Echo zshrc config
echo '{}' > ~/.zshrc
    "##, zshrc_string)
}
