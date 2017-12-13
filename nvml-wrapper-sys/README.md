# nvml-wrapper-sys

[![Crates.io version](https://img.shields.io/crates/v/nvml-wrapper-sys.svg?style=flat-square)](https://crates.io/crates/nvml-wrapper-sys)
[![Crates.io downloads](https://img.shields.io/crates/d/nvml-wrapper-sys.svg?style=flat-square)](https://crates.io/crates/nvml-wrapper-sys)
[![Docs.rs docs](https://docs.rs/nvml-wrapper-sys/badge.svg)](https://docs.rs/nvml-wrapper-sys)

Rust bindings for the
[NVIDIA Management Library](https://developer.nvidia.com/nvidia-management-library-nvml)
(NVML), a C-based programmatic interface for monitoring and managing various states within
NVIDIA (primarily Tesla) GPUs.

It is intended to be a platform for building 3rd-party applications, and is also the
underlying library for NVIDIA's nvidia-smi tool.

NVML supports the following platforms:

* Windows
  * Windows Server 2008 R2 64-bit
  * Windows Server 2012 R2 64-bit
  * Windows 7 64-bit
  * Windows 8 64-bit
  * Windows 10 64-bit
* Linux
  * 64-bit
  * 32-bit
* Hypervisors
  * Windows Server 2008R2/2012 Hyper-V 64-bit
  * Citrix XenServer 6.2 SP1+
  * VMware ESX 5.1/5.5

And the following products:

* Full Support
  * Tesla products Fermi architecture and up
  * Quadro products Fermi architecture and up
  * GRID products Kepler architecture and up
  * Select GeForce Titan products
* Limited Support
  * All GeForce products Fermi architecture and up

## Compilation

The NVML library comes with the NVIDIA drivers and is essentially present on any
system with a functioning NVIDIA graphics card. The compilation steps vary
between Windows and Linux, however.

### Windows

I have been able to successfully compile and run the tests for these bindings' wrapper
using both the GNU and MSVC toolchains. An import library (`nvml.lib`) is included for
compilation with the MSVC toolchain.

The NVML library dll can be found at `%ProgramW6432%\NVIDIA Corporation\NVSMI\`
(which is `C:\Program Files\NVIDIA Corporation\NVSMI\` on my machine). I had to add
this folder to my `PATH` or place a copy of the dll in the same folder as the executable
in order to have everything work properly at runtime with the GNU toolchain. You may
need to do the same; I'm not sure if the MSVC toolchain needs this step or not.

### Linux

The NVML library can be found at `/usr/lib/nvidia-<driver-version>/libnvidia-ml.so`; on my
system with driver version 375.51 installed, this puts the library at
`/usr/lib/nvidia-375/libnvidia-ml.so`. You will need to create a symbolic link:

```bash
sudo ln -s /usr/lib/nvidia-<driver-version>/libnvidia-ml.so /usr/lib
```

## NVML Support

These bindings were generated for NVML version 9. Each new version of NVML is
guaranteed to be backwards-compatible according to NVIDIA, so these bindings
should be useful regardless of NVML version bumps.

## Rust Version Support

Currently supports Rust 1.19.0 or greater. The target version is the **latest**
stable version; I do not intend to pin to an older one at any time.

## License

Licensed under either of

* Apache License, Version 2.0
   ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
* MIT license
   ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.
