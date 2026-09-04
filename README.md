# twlvi

Print the lyrics to _The Twelve Days of Christmas_ in sequence

## Why?
This was built to get some practice with concepts from chapters 1-3 of the rust
book.

## Prerequisites
[Cargo](https://doc.rust-lang.org/cargo/getting-started/installation.html)
or
[Nix](https://nixos.org/download/)

## Compatibility
It's rust so it should just work on any of the big three? There's some docs on
Windows exceptions I ain't quite read yet...

## Usage
Enter a number between 1 and 12 when prompted or enter `f` to print the full
sequence and exit.

At the prompt, use `q` to quit.
At any point during runtime use `ctrl-c` to exit.

## Uninstall
**Cargo**
```sh
rm -r /path/to/twlvi
```

**Nix**
```sh
nix-store --gc
```

## Run without installing
**Cargo**

Run the program from within the project root:
```sh
git clone git@github.com/fuguesoft/twlvi
cd /path/to/twlvi
cargo run
```

**Nix**
1. Enable Nix Flakes ([See Below](##Install))
2. Temporarily build and run without installing
```sh
nix run github:fuguesoft/twlvi
```

## Install
**Cargo**

Download the repository:

```sh
git clone git@github.com/fuguesoft/twlvi
```

Build and install:
```sh
cd /path/to/twlvi
cargo build
```

**Nix/NixOS (Flake)**

1. Enable nix flakes in `configuration.nix` 
```nix
nix.settings.experimental-features = [ "nix-command" "flakes" ];
```

or either `/etc/nix/nix.conf` or `$XDG_CONFIG_HOME/nix/nix.conf`
```conf
experimental-features = nix-command flakes
```

2. Add flake input to `/etc/flake.nix`
```nix

inputs = {
  # ...
  twlvi = {
    url = "github:fuguesoft/twlvi"
      };
  # ...
};
```

3. Expose flake input in `configuration.nix` or `home.nix`
configuration.nix
```nix
environment.systemPackages = with pkgs; [
  inputs.twlvi.packages."${pkgs.stdenv.hostPlatform.system}".default
]
```

home.nix
```nix
home.packages = with pkgs; [
  inputs.twlvi.packages."${pkgs.stdenv.hostPlatform.system}".default
]
```
