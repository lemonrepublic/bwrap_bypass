bwrap_bypass

bwrap_bypass is a lightweight, high-performance toolkit designed to intercept, sanitize, configure, and extend Bubblewrap (bwrap) executions on Linux. It provides TOML-driven bind-mount injection, environment whitelisting/blacklisting, debug logging, and dynamic library preloading (LD_PRELOAD) hooks.

FEATURES
- TOML Configuration Loader: Dynamically load custom read-only (--ro-bind) and read-write (--bind) bind mounts, environment rules, and bypass settings from bwrap_bypass.toml.
- Environment Sanitization & Filtering: Strict whitelist and blacklist controls over environment variables.
- Namespace Stripping: Optional bypass flags to strip restrictive sandbox arguments (such as --unshare-user).
- Debug Logging Mode: Verbose execution tracking enabled via BWRAP_BPASS_DEBUG=1.
- Flatpak Integration: Helper wrapper script to transparently apply rules to individual Flatpak applications.
- Multi-Architecture Support: Native cross-compilation pipeline supporting 9 distinct CPU architectures.

PACKAGE MANAGER PREREQUISITES
Depending on your distribution and whether you are compiling locally or cross-compiling, ensure your system has the appropriate build prerequisites installed:

Debian / Ubuntu / Linux Mint / Pop!_OS / Kali Linux
- Rust Toolchain & Cargo: curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
- Build Essentials: sudo apt install build-essential pkg-config

Void Linux
- Rust Toolchain & Cargo: curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
- Build Essentials: sudo xbps-install -S base-devel pkg-config

Arch Linux / Manjaro / EndeavourOS
- Rust Toolchain & Cargo: curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
- Build Essentials: sudo pacman -S base-devel pkg-config

Fedora / RHEL / CentOS Stream / Rocky Linux / AlmaLinux
- Rust Toolchain & Cargo: curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
- Build Essentials: sudo dnf groupinstall "Development Tools" && sudo dnf install pkg-config

openSUSE Leap / Tumbleweed
- Rust Toolchain & Cargo: curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
- Build Essentials: sudo zypper install -t pattern devel_basis && sudo zypper install pkg-config

NixOS
- Rust Toolchain & Cargo / Build Essentials: Available via nix-shell or flake (gcc, pkg-config, rustup)

Gentoo Linux
- Rust Toolchain & Cargo: curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
- Build Essentials: sudo emerge --ask sys-devel/base-development dev-util/pkgconf

Alpine Linux
- Rust Toolchain & Cargo: curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
- Build Essentials: sudo apk add build-base pkgconf

Cross-Compilation Tool (cross) [All Distributions]
- Required for building across multi-architecture targets:
  cargo install cross --locked
- Container Runtime: Docker or Podman (required by cross to run containerized target toolchains).

TARGET ARCHITECTURES SUPPORTED
The project supports cross-compilation for the following 9 target platforms:
- amd64 (x86_64-unknown-linux-gnu): 64-bit PC / x86-64 (AMD and Intel 64-bit).
- arm64 (aarch64-unknown-linux-gnu): 64-bit ARM (AArch64, ARMv8 servers & single-board computers).
- armhf (armv7-unknown-linux-gnueabihf): 32-bit ARM (Hard Float ABI, ARMv7 with VFPv3).
- armel (arm-unknown-linux-gnueabi): 32-bit ARM (soft-float, legacy embedded devices).
- ppc64el (powerpc64le-unknown-linux-gnu): 64-bit Little-Endian PowerPC (POWER8, POWER9+).
- riscv64 (riscv64gc-unknown-linux-gnu): 64-bit RISC-V architecture.
- s390x (s390x-unknown-linux-gnu): 64-bit IBM Z / S390x enterprise mainframes.
- loong64 (loongarch64-unknown-linux-gnu): 64-bit little-endian LoongArch architecture.
- i386 (i686-unknown-linux-gnu): 32-bit PC (historical IA-32 / x86).

INSTALLATION & BUILDING
Local Build & Installation:
   git clone https://github.com/lemonrepublic/bwrap_bypass.git
   cd bwrap_bypass
   make release
   make test
   sudo make install

Multi-Architecture Cross-Compilation (dist/ folder):
   make multi-arch

CONFIGURATION (bwrap_bypass.toml)
[bypass]
target_bwrap = "/usr/bin/bwrap"
enable_sanitization = true
unshare_user_bypass = false

[binds]
read_only = [
    "/usr/share/fonts",
    "/etc/fonts"
]
read_write = []

[environment]
whitelist = ["LD_LIBRARY_PATH", "PATH", "HOME", "USER", "DISPLAY", "WAYLAND_DISPLAY"]
blacklist = ["LD_PRELOAD_UNTRUSTED"]

USAGE & DEBUGGING
- Enable Debug Mode:
  BWRAP_BPASS_DEBUG=1 fake_bwrap --help
- Run via Flatpak Wrapper:
  ./scripts/flatpak-wrapper com.spotify.Client

LICENSE
Distributed under the MIT License. See LICENSE for details.
