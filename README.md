# Rusttale

<img width="1536" height="1024" alt="rusttale, undertale, deltarune, rougelike, rpg" src="https://github.com/user-attachments/assets/bcd2f14a-29a9-42b7-8051-181c6466407d" />

[![](https://dcbadge.limes.pink/api/server/QRjRkt65Ht)](https://discord.gg/QRjRkt65Ht)

**Rusttale** is a 2D game developed using the [Tetra](https://github.com/tetra-engine/tetra) engine in Rust.

## Roadmap

- [ ] CI/CD release builds
- [ ] Prebuild install scripts
- [ ] In-game menu system

## Localization Support

There is currently limited localization support. It does not restrict language entries or context length, but there is no support for variables or custom values yet. Every entry must be constant. On the bright side, creating new transcriptions is very straightforward.

## Sound & Music

- Powered by [SuperCollider](https://supercollider.github.io)

## Installation

### Build from Source

1. **Clone the repository:**
```sh
git clone https://github.com/lazypwny751/rusttale.git && cd rusttale
```

2. **Fetch resources:**
```sh
cargo run -p resources
```

3. **Build for release:**
```sh
cargo build --release
```

4. **Run:**
```sh
./target/release/rusttale
```

### Prebuild Installation (Coming Soon)

```sh
curl -S <link> | bash -
```

## Documentation & Manpages

This project includes manual pages. See [docs/man/README.md](docs/man/README.md) for installation instructions.

## Usage

```console
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
