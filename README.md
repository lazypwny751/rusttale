# rusttale

![image](https://us1.discourse-cdn.com/flex019/uploads/rust_lang/original/2X/9/9f76ef5e791e27deaaafbca2a3bea35d63e165c8.gif)

[![](https://dcbadge.limes.pink/api/server/QRjRkt65Ht)](https://discord.gg/QRjRkt65Ht)

# Project.

## TODO:
- CI/CD release builds.
- prebuild install scripts.
- menu.

## localization support.
There's a limited localization support, that's not limiting the language entry or context lenght but there is no variables or any custom values, so every entry must be constant, and on the another side, creating this transcription is too easy.


## Sound&Music.
- [supercollider](https://supercollider.github.io)

# Installation.

Build from sources
```sh
# Let's clone the repository to local.
git clone https://github.com/lazypwny751/rusttale.git && cd rusttale

# First get the resources.
cargo run -p resources

# Build a release.
cargo build --release
```

or via prebuild installation script.

```sh
curl -S <link> | bash -
```

# Usage.
```
Usage: rusttale [OPTIONS]

Options:
  -l, --language <LANGUAGE>  [default: en]
      --mode <MODE>          [default: game]
  -h, --help                 Print help
  -V, --version              Print version
```

# Contributing.
Pull requests are welcome. For major changes, please open an issue first to discuss what you would like to change.

# License
[MIT](https://choosealicense.com/licenses/mit/)
