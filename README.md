Bwrap Bypass Ecosystem
A Rust-based toolkit to bypass bubblewrap (bwrap) sandbox constraints in unprivileged PRoot, Android Termux, and container environments where CLONE_NEWUSER namespaces are restricted.
Architecture
    • libbwrap_bypass.so: A dynamic library (LD_PRELOAD) hook targeting execve to intercept and rewrite sandbox calls.
    • fake_bwrap: A drop-in binary override utility that strips out unsupported sandbox arguments and passes through the underlying executable payload.
    • flatpak-wrapper: A CLI launcher that automatically injects the LD_PRELOAD environment variable to run desktop applications smoothly.

Prerequisites: Installing Cargo & Rust
Before building the project, ensure you have the Rust toolchain (cargo and rustc) installed on your Linux distribution using your package manager:
Debian / Ubuntu / Linux Mint
sudo apt update
sudo apt install cargo rustc build-essential
Arch Linux / Manjaro
sudo pacman -S rust base-devel
Void Linux
sudo xbps-install -S rust base-devel cargo
Fedora / RHEL / CentOS
sudo dnf install cargo rustc make gcc
openSUSE Leap / Tumbleweed
sudo zypper install rust cargo make gcc
Gentoo Linux
sudo emerge --ask dev-lang/rust sys-devel/make sys-devel/gcc
Slackware
slackpkg update
slackpkg install rust
Universal Method (Rustup - Recommended)
If your distribution's packaged version is outdated, you can install the official toolchain universally:
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source "$HOME/.cargo/env"

How to Build and Install
Once cargo is installed, compile the release binaries and install them into your system path:
# 1. Navigate to your repository directory
cd bwrap_bypass

# 2. Build release binaries and dynamic library
cargo build --release

# 3. Install binaries and libraries to system paths (requires root/sudo)
sudo make install
Verification
Verify that the ecosystem components were successfully installed:
ls -lh /usr/local/lib/libbwrap_bypass.so
which fake_bwrap
which flatpak-wrapper
