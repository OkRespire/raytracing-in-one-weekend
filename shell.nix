{pkgs ? import <nixpkgs> {}}:



pkgs.mkShell {
    packages = with pkgs; [
        rustup
        rust-analyzer
    ];
}

