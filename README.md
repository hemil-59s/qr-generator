# ⚡ QR Generator

> Blazing-fast QR code generator built with **Rust** — the language used by Microsoft, Discord & Linux Kernel.

## 🚀 Usage

```bash
# Generate PNG
cargo run -- --text "https://yourlink.com" --output qr.png

# Generate SVG
cargo run -- --text "https://yourlink.com" --output qr.svg
```

## ⚙️ Install

```bash
git clone https://github.com/hemil-59s/qr-generator
cd qr-generator
cargo build --release
```

## 🛠️ Built With
- **Rust** — Systems programming language
- **qrcode-rs** — QR encoding library
- **image-rs** — Image processing

## 📄 License
MIT © [hemil-59s](https://github.com/hemil-59s)
