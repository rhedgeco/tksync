# Typekit Sync
This is an unofficial tool for downloading fonts from Adobe Fonts (Typekit). You must have a valid adobe account to use this tool as it takes advantage of the Web Projects feature.

# Usage
### Sync all projects
To sync all projects just run:
```
tksync
```

### Add new Typekit Project
To add a new project to be tracked by tksync, run the following:
```
tksync add [OPTIONS] <ID> <NAME> <PATH>

Arguments:
  <ID>    Id of the typekit project
  <NAME>  Name of typekit project
  <PATH>  Path to download project fonts to

Options:
  -r, --replace  Overwrite existing project id if it exists
```
# Installation
### Arch Linux
```
sudo pacman -S --needed base-devel
git clone https://aur.archlinux.org/tksync.git
cd tksync
makepkg -si
```
### Arch Linux using [paru](https://github.com/Morganamilo/paru)
```
paru -S tksync
```
### Manual Install
Install the `rust` package manager [cargo](https://doc.rust-lang.org/cargo/getting-started/installation.html)

Then build the binary
```
git clone https://github.com/rhedgeco/tksync.git
cd tksync
cargo build --release
```

Next, copy the binary to a place of your choosing (probably somewhere in your shells PATH):
```
cp ./target/release/tksync /usr/bin/tksync
```

## [MIT LICENSE](LICENSE.md)