Rust wrapper to [MATLAB MAT file I/O library](https://github.com/tbeu/matio)


This crate provides bindings and wrappers for [MATIO](https://github.com/tbeu/matio):
MATLAB MAT file I/O C library

# Dependencies

- cmake
- zlib

## Installing dependencies on Ubuntu/Debian

```sh
sudo apt install cmake zlib1g-dev
```

## Installing dependencies on Windows

Download and install [LLVM 15.0.7](https://github.com/llvm/llvm-project/releases/tag/llvmorg-15.0.7)

```powershell
winget install cmake
vcpkg install zlib
```

Set environment variable ZLIB_LIB_DIR to `<vcpkg>\packages\zlib_x64-windows\lib`
