# Markdown to HTML Converter

A CLI Tool written in Rust that converts Markdown files into styled HTML or PDF documents. Features a live-reloading mode "--live" with a built-in web server for real-time previews.

## Installation

1. **Clone the repository**:
   ```bash
   git clone https://github.com/viemmsakh/mdc.git
   cd mdc
   ```

2. **Build with Cargo**
    ```bash
    cargo build --release
    ```

## Usage
Run the progam without arguments to see the help screen

**Basic Conversion**
_Outputs a raw HTML snippet to the terminal:_
```bash
./mdc --input README.md
```

**Save to File with Styling**
_Wraps the content in a full HTML document and saves it to a specific path, infers exension if not given:_
```bash
./mdc --input README.md --output test/readme --html
```

**Save to PDF**
_Generates a PDF and saves it to a specific path, infers exension if not given:_
```bash
./mdc --input README.md --output test/readme --pdf
```

**Live Preview**
_Starts a local server and watches for file changes. Everytime you save your markdown file, the browsers at `http://0.0.0.0:3000` will refresh instantly:_
```bash
./mdc --input README.md --live
```

## Command Line Arguments
| Flag | Short | Description |
| --- | --- | --- |
| `--input` | `-i` | **Required.** Path to the input `.md` file. |
| `--output` | `-o` | Path to the output file. |
| `--html` | | Wraps the output in a styled HTML boilerplate. |
| `--pdf` | | Outputs markdown to PDF. |
| `--live` | `-l` | Watches input for changes and displays updates live in browser. |
| `--port` | `-p` | Port to serve the live preview on (default: 3000). |
| `--help` | `-h` | Displays help information. |
| `--version` | `-V` | Displays version information. |

## Author
Aaron Glaza

## License
This project is licensed under the MIT License.