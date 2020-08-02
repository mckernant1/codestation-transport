use std::process::Command;

pub fn export_p10k() -> String {
    let p10k_output = match Command::new("cat")
        .arg(format!("{}/.p10k.zsh", env!("HOME")))
        .output() {
        Ok(x) => x,
        Err(e) => {
            panic!("{:?}", e)
        }
    };
    let p10k_string =
        String::from_utf8_lossy(&p10k_output.stdout)
            .replace("'", "\"");

    return format!(r##"
# Install p10k
git clone --depth=1 https://github.com/romkatv/powerlevel10k.git ${{ZSH_CUSTOM:-$HOME/.oh-my-zsh/custom}}/themes/powerlevel10k

# Echo p10k config
echo '{}' > ~/.p10k.zsh
    "##, p10k_string)
}
