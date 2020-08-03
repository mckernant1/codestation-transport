# Codestation-Transport

Creates an sh file that will export your current setup for various utilities.

## Included Utilities
- brew
- npm
- zsh (.zshrc + oh-my-zsh installation)
- p10k (.p10k.zsh + p10k installation via oh-my-zsh)

## Install
```bash
brew tap mckernant1/tools
brew install codestation-transport
```

## How to use
```bash
codestation-transport brew npm
```
You should export this to a file that you move to your new machine
```bash
codestation-transport brew npm > init.sh
```

For example my output looks like this
```bash
# ====== BREW ======
# Install xcode
xcode-select --install

# Install homebrew
/bin/bash -c "$(curl -fsSL https://raw.githubusercontent.com/Homebrew/install/master/install.sh)"


# All your taps
brew tap dart-lang/dart
brew tap holgerbrandl/tap
brew tap homebrew/cask
brew tap homebrew/cask-fonts
brew tap homebrew/cask-versions
brew tap homebrew/core
brew tap homebrew/test-bot
brew tap mckernant1/tools
brew tap rs/tap

# All your formulae
brew install adns aom auto-updater autojump awscli bat bdw-gc cairo case-randomizer codestation-transport cscope curl darksky-weather dav1d ffmpeg flac fontconfig freetype frei0r fribidi fswatch gcc gdb gdbm gettext ghc ghostscript giflib git glib gmp gnupg gnutls go gradle graphite2 grip guile harfbuzz htop icu4c isl jaggr jpeg jplot jq kotlin kscript kubernetes-cli lame leptonica less libass libassuan libbluray libevent libffi libgcrypt libgpg-error libidn2 libksba libmpc libogg libpng libsamplerate libsndfile libsoxr libssh2 libtasn1 libtiff libtool libunistring libusb libvidstab libvorbis libvpx libyaml little-cms2 lua lzo macvim make mpfr ncurses nettle npth nushell nvm oniguruma openblas opencore-amr openjdk openjpeg openssl@1.1 opus p11-kit pcre pcre2 perl pinentry pipenv pixman pkg-config project-manager python@3.8 rav1e readline rlwrap rtmpdump rubberband ruby rust rustup-init sbt scala sdl2 sl snappy speex sqlite srt streamlink telnet tesseract theora timeit tmux trash tree unbound utf8proc uwucat vegeta webp wget x264 x265 xvid xz youtube-dl zsh

# All your casks
brew cask install adoptopenjdk8 amazon-chime atom box-sync discord docker firefox flux font-meslo-for-powerline google-backup-and-sync google-chrome intellij-idea iterm2 league-of-legends malwarebytes postman spotify sublime-text twitch visual-studio-code vlc



# Install Global NPM dependencies
npm i -g @angular/cli @vue/cli nodemon npm serve
```
