bwrap_bypass

bwrap_bypass is a lightweight ecosystem designed to intercept, wrap, and bypass bwrap (Bubblewrap) restrictions on Linux distributions like Void Linux. This repository provides preloaded libraries and wrapper binaries to ensure seamless application execution (such as Flatpak packages) in restricted environments.

Features

Dynamic Interception (libbwrap_bypass.so): Uses LD_PRELOAD and RTLD_NEXT to hook system calls and redirect calls dynamically.

Custom Wrapper (fake_bwrap): Acts as a drop-in replacement for bubblewrap, forwarding arguments safely to the real /usr/bin/bwrap.

Flatpak Integration (flatpak-wrapper): Streamlines launching sandboxed desktop applications like GNOME Calculator under the custom bypass layer.

Installation

Prerequisites

Rust and Cargo

GCC / Clang / build-essentials

Bubblewrap (bwrap) version 0.13.1 or compatible

Building from Source

Clone the repository:
git clone https://github.com/lemonrepublic/bwrap_bypass.git
cd bwrap_bypass

Build the project using Cargo:
cargo build --release

Install binaries and libraries to your system paths:
sudo cp target/release/libbwrap_bypass.so /usr/local/lib/
sudo cp target/release/fake_bwrap /usr/local/bin/
sudo cp scripts/flatpak-wrapper /usr/local/bin/

Usage

To run applications with the bypass layer active, use the provided wrapper script:

flatpak-wrapper run org.gnome.Calculator

Manual Environment Preloading
You can also manually preload the bypass library into any target command:
LD_PRELOAD=/usr/local/lib/libbwrap_bypass.so flatpak run org.gnome.Calculator

Troubleshooting & Debugging

You can use GDB to trace and inspect how the wrapper handles process spawns and system calls:
gdb --args flatpak-wrapper run org.gnome.Calculator

Inside GDB:
set follow-fork-mode child
set detach-on-fork off
set environment LD_PRELOAD=/usr/local/lib/libbwrap_bypass.so
break execve
run

Contributing

Pull requests are welcome! For major changes, please open an issue first to discuss what you would like to change.
