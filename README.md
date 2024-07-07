# ytmdlink

`ytmdlink` is a simple CLI program written in Rust that fetches YouTube oEmbed data and generates markdown links. The program supports two formats of markdown links: inline and reference.

## Features

- Fetches YouTube oEmbed data
- Generates markdown links in `[title](url)` format
- Generates markdown reference links in `[^X]: url "title"` format

## Installation

To install `ytmdlink`, you need to have Rust installed on your system. If you don't have Rust installed, you can download it from [rust-lang.org](https://www.rust-lang.org/).

Clone the repository and build the project:

```sh
git clone <repository-url>
cd ytmdlink
cargo build --release
```

After building the project, you can find the binary in the `target/release` directory.

## Usage

To use `ytmdlink`, run the following command:

```sh
./ytmdlink [OPTIONS] <URL>
```

### Arguments

- `<URL>`: The YouTube URL you want to generate a markdown link for.

### Options

- `-l, --link-type <LINK_TYPE>`: The type of markdown link output. It can be either `link` (default) for `[title](url)` format or `ref` for `[^X]: url "title"` format.

- `-h, --help`: Print help information.

### Examples

Generate an inline markdown link:

```sh
./ytmdlink https://www.youtube.com/watch?v=dQw4w9WgXcQ --link-type link
```

Output:

```
[Rick Astley - Never Gonna Give You Up - Official Music Video](https://www.youtube.com/watch?v=dQw4w9WgXcQ)
```

Generate a reference markdown link:

```sh
./ytmdlink https://www.youtube.com/watch?v=dQw4w9WgXcQ --link-type ref
```

Output:

```
[^X]: https://www.youtube.com/watch?v=dQw4w9WgXcQ "Rick Astley - Never Gonna Give You Up - Official Music Video"
```

## Contributing

Contributions are welcome! If you have any ideas, suggestions, or bug reports, please open an issue or submit a pull request.

1. Fork the repository
2. Create a new branch (`git checkout -b feature/your-feature`)
3. Commit your changes (`git commit -am 'Add some feature'`)
4. Push to the branch (`git push origin feature/your-feature`)
5. Create a new Pull Request

## License

This project is licensed under the MIT License. See the [LICENSE](LICENSE) file for more details.

## Acknowledgements

- [anyhow](https://github.com/dtolnay/anyhow) for error handling
- [clap](https://github.com/clap-rs/clap) for command-line argument parsing
- [reqwest](https://github.com/seanmonstar/reqwest) for making HTTP requests
- [serde_json](https://github.com/serde-rs/json) for parsing JSON responses
