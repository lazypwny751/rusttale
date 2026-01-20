# The Lore

Welcome to the Lore section of our documentation! Here, you'll find a wealth of information about the background stories, characters, and world-building elements that enrich our project. Dive into the narratives that shape the universe we are creating and explore the intricate details that bring our story to life.

> [!NOTE]
> building the lore book from using build.sh script <br>
> but you need to install **pdflatex** first. <br>
> For debian based system, you can use:
> ```sh
> sudo apt install texlive-latex-base
> ```
> for arch linux and derivatives:
> ```sh
> sudo pacman -S texlive-core
> ```
> for fedora based system:
> ```sh
> sudo dnf install texlive
> ```
> and finally for macOS using homebrew:
> ```sh
> brew install mactex-no-gui
> ```

Once you have **pdflatex** installed, you can run the build script to generate the lore book in PDF format. This will compile all the lore content into a single, cohesive document that you can easily read and share.

## Building the Lore Book
To build the lore book, simply navigate to the `docs/the_lore/` directory in your terminal and execute the following command:

subcommands:
- `build` : build the lore book in the specified language
- `clean` : remove generated files

options of the build.sh script:
- `-v` : display verbose output during the build process
- `-p <path>` : prefix path to save the generated PDF file (default: current directory)
- `-h` : display help message

```sh
./build.sh [subcommand] [<options>] <language_code>
```

an example to build the lore book in English:
```sh
./build.sh build en
```

> [!IMPORTANT]
> By default the script will generate automatically the lore book in English (`en`) and Turkish (`tr`), but you can specify other languages if available.

# Contributing to the Lore
We welcome contributions to the lore! If you have ideas for new stories, characters, or world-building elements, please feel free to submit a [pull request](https://github.com/lazypwny751/rusttale) or open an [issue](https://github.com/lazypwny751/rusttale/issues) on our GitHub repository. Your creativity and passion help make our project even more engaging and immersive.