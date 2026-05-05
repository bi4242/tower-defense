{ pkgs ? import <nixpkgs> {} }:

let
  fenix = import (fetchTarball "https://github.com/nix-community/fenix/archive/main.tar.gz") {};
  toolchain = fenix.combine [
    fenix.stable.rustc
    fenix.stable.cargo
    fenix.targets.x86_64-pc-windows-gnu.stable.rust-std
  ];
  mingw = pkgs.pkgsCross.mingwW64.stdenv.cc;
  mcfgthread = "/nix/store/gb95ngv9c1brgjjkzhnynl2366syk6xn-mcfgthread-x86_64-w64-mingw32-2.3.1";
in pkgs.mkShell {
  buildInputs = [ toolchain mingw ];

  CARGO_TARGET_X86_64_PC_WINDOWS_GNU_LINKER =
    "${mingw}/bin/x86_64-w64-mingw32-gcc";

  shellHook = ''
    export RUSTC="${toolchain}/bin/rustc"
    export CARGO="${toolchain}/bin/cargo"

    # symlink mcfgthread as libpthread.a in a temp dir
    mkdir -p /tmp/mingw-pthread
    ln -sf ${mcfgthread}/lib/libmcfgthread.a /tmp/mingw-pthread/libpthread.a
  '';
}