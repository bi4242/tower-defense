{ pkgs }:
let
  inherit (pkgs) lib;
in
pkgs.mkShell {
  buildInputs = with pkgs; [
    rustc
    cargo
    rustfmt
    rustPackages.clippy
    libGL
    libdrm
    xorg.libX11
    xorg.libXrandr
    xorg.libXxf86vm
    libxcb
    libxkbcommon
    vulkan-loader
    mesa

    typst
  ];

  LD_LIBRARY_PATH = lib.makeLibraryPath (with pkgs; [
    rustc
    cargo
    libGL
    libdrm
    xorg.libX11
    xorg.libXrandr
    xorg.libXxf86vm
    xorg.libXcursor
    xorg.libXi
    libxcb
    libxkbcommon
    vulkan-loader
    mesa
  ]);

  RUST_BACKTRACE = 1;

  shellHook = ''
    ulimit -n 8192
  ''; # Neovim may hit the file descriptor limit.
}
