| :warning: WARNING                                        |
|:--------------------------------------------------------:|
| This project is incomplete and may not work as expected. |

# oxidize ![](https://github.com/m1ten/oxidize/workflows/Rust/badge.svg?branch=main) [![Rustc Version]][rustc] <a href="https://discord.gg/Qy7QdamBSN"> <img src="https://discord.com/assets/e4923594e694a21542a489471ecffa50.svg" width="75"/> </a>

[Rustc Version]: https://img.shields.io/badge/rustc-1.53.0.nightly-lightgray.svg
[rustc]: https://github.com/rust-lang/rust/milestone/81

one of the many build systems

## Installation

Download the latest binary from [releases](https://github.com/m1ten/oxidize/releases) (stable) or [actions](https://github.com/m1ten/oxidize/actions) (individual commits; *You will need to be logged in to download the artifacts*)

```sh
# Linux and macOS: Give execution permission to oxidize and run
$ chmod +x oxidize && ./oxidize

# Windows: Run the exe
$ ./oxidize.exe
```
### Building from source 

1. Install [Rust nightly >=1.53.0](https://github.com/rust-lang/rust/milestone/81) using [`rustup`](https://www.rust-lang.org/tools/install)
   ```sh
   $ rustup toolchain install nightly
   ```
2. Clone the [source](https://github.com/m1ten/oxidize) using [`git`](https://git-scm.com/)
    ```sh
    $ git clone https://github.com/m1ten/oxidize.git
    $ cd oxidize
    ```
3. Build and run using [`cargo`](https://doc.rust-lang.org/stable/cargo/)
    ```sh
    $ cargo +nightly run --release
    ```

## License

oxidize is licensed under [zlib](./LICENSE).
