# GooeyGit

A minimalist GUI Git client built with Rust, GTK4, and libadwaita.

## Features

- Minimalist, clean interface following GNOME's Human Interface Guidelines
- Repository browsing with branch and remote management
- File status viewing (staged/unstaged changes)
- Commit creation and history viewing
- Basic Git operations (pull, push, branch management)

## Dependencies

- Rust
- GTK4
- libadwaita
- libgit2

## Building

Make sure you have the required dependencies installed on your system.

### Linux

Install GTK4 and libadwaita development libraries:

```bash
# For Fedora
sudo dnf install gtk4-devel libadwaita-devel libgit2-devel

# For Ubuntu/Debian
sudo apt install libgtk-4-dev libadwaita-1-dev libgit2-dev

# For Arch Linux
sudo pacman -S gtk4 libadwaita libgit2
```

Then, build the application:

```bash
cargo build --release
```

The built binary will be available at `target/release/gooey-git`

## Usage

1. Launch the application
2. Open a Git repository using the folder icon in the header
3. View and manage Git operations through the intuitive interface

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

## License

This project is licensed under the MIT License - see the LICENSE file for details.
