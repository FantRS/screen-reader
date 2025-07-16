# Screen reader

`screen-reader` is a small command-line utility for Linux written in Rust that lets you capture a selected area of the screen, extract text from the image using OCR (Tesseract via `rusty-tesseract`), and copy the recognized text to the system clipboard.

---

## ✨ Features

- Rust implementation for Linux.
- Uses `maim` for interactive screenshot capture.
- OCR support for English, Ukrainian, and Russian.
- Outputs recognized text directly into the clipboard.

![screen](https://github.com/user-attachments/assets/eabba2b1-7fb2-49c3-be55-cccba470dd7b)


---

## 📄 Requirements

Make sure the following are installed:

- `maim` (https://github.com/naelstrof/maim)
- `xclip`
- `tesseract-ocr` (https://github.com/tesseract-ocr/tesseract)
- `tesseract-ocr` language data for: `eng`, `ukr`, `rus`

---

## 🚀 Getting Started

### Install OCR Engine and language data

```bash
sudo apt install tesseract-ocr tesseract-ocr-eng tesseract-ocr-ukr tesseract-ocr-rus
```

### Install maim and xclip utilities

```bash
sudo apt install maim xclip
```

### Download binary from releases
https://github.com/FantRS/screen-reader/releases/download/screen/screen-reader

### Run binary application
```bash
./screen-reader
```

---

## 🛠 Build and Run

Clone the repository:

```bash
git clone https://github.com/5antUA/screen-reader.git
cd screen-reader
```

Build the project:

```bash
cargo build --release
```

Run the application:

```bash
cargo run --release
```

When you run it, a crosshair will appear for you to select the screen area. After selection:
- A screenshot is saved.
- Text is extracted with OCR.
- The recognized text is automatically copied to your clipboard.

---

## 👨‍💻 Author
**Rostyslav Kashper**  
Rust dev / Game dev  
GitHub: _[FantRS](https://github.com/FantRS)_
