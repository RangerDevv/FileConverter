# FileFlow

A powerful, local file converter built with Tauri and Svelte. Convert images, PDFs, text files, data files, and more - all processed locally on your machine for privacy and speed.

![FileFlow](https://img.shields.io/badge/Made%20with-Tauri%20%2B%20Svelte-blue)

## Features

### Supported Conversions

#### 🖼️ Images
- **Formats**: PNG, JPG/JPEG, GIF, WebP, BMP, ICO, TIFF, AVIF
- Convert between any supported image format
- Adjust JPEG quality
- Preview images before conversion

#### 📄 PDF
- Extract text from PDFs
- View page count and PDF version

#### 📝 Text & Code Files
- **Formats**: TXT, MD, HTML, JSON, XML, YAML, and more
- Convert to PDF, HTML, Markdown, or plain text
- Syntax-aware preview

#### 📊 Data Files
- **CSV ↔ JSON** conversion
- CSV to HTML table
- Data preview

### Key Features

- **100% Local** - All processing happens on your machine
- **Fast** - Native Rust backend for blazing fast conversions
- **Clean UI** - Modern, intuitive interface
- **Drag & Drop** - Simply drop files to convert
- **Preview** - See your files before converting
- **Cross-Platform** - Works on Windows, macOS, and Linux

## Installation

### Prerequisites

- [Node.js](https://nodejs.org/) (v18 or higher)
- [Rust](https://www.rust-lang.org/tools/install) (latest stable)
- Platform-specific dependencies for Tauri:
  - **Linux**: `sudo apt install libwebkit2gtk-4.1-dev build-essential curl wget file libssl-dev libayatana-appindicator3-dev librsvg2-dev`
  - **macOS**: Xcode Command Line Tools
  - **Windows**: Microsoft Visual Studio C++ Build Tools

### Development

```bash
# Clone the repository
git clone <repo-url>
cd FileConverter

# Install dependencies
npm install

# Run in development mode
npm run tauri dev

# Build for production
npm run tauri build
```

## Usage

1. **Select a file** - Drag and drop a file or click to browse
2. **Choose output format** - Select from available conversion formats
3. **Set output location** - Choose where to save the converted file
4. **Convert** - Click "Convert & Download" button
5. **Open result** - Click "Open Folder" to view the converted file

## Architecture

- **Frontend**: Svelte 5 with modern reactive patterns
- **Backend**: Rust with Tauri 2.0
- **Image Processing**: `image` crate for comprehensive format support
- **PDF Processing**: `lopdf` for reading, `printpdf` for writing, `pdf-extract` for text extraction
- **Data Processing**: `csv` and `serde_json` for data format conversions

## License

MIT License
