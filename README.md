# ⚡ QR Generator — Built in Rust

> Blazing-fast QR code generator built with **Rust** — the language used by Microsoft, Discord & Linux Kernel.
> Generate PNG and SVG QR codes directly from your terminal.

---

## 🚀 Quick Start

### 1. Install Rust
```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```
> Windows users: download from [rustup.rs](https://rustup.rs)

### 2. Clone & Build
```bash
git clone https://github.com/hemil-59s/qr-generator
cd qr-generator
cargo build --release
```

### 3. Generate QR Code
```bash
# Any URL
cargo run -- --text "https://yourlink.com" --output qr.png

# SVG format
cargo run -- --text "https://yourlink.com" --output qr.svg

# WhatsApp
cargo run -- --text "https://wa.me/919876543210" --output whatsapp.png
```

---

## ⚙️ Options

| Flag | Default | Description |
|------|---------|-------------|
| `--text` | required | URL or text to encode |
| `--output` | `qr.png` | Output file (.png or .svg) |

---

## 🛠️ Built With
- **Rust** — Systems programming language
- **qrcode-rs** — QR encoding
- **image-rs** — Image processing

---

## 📄 License
MIT © [hemil-59s](https://github.com/hemil-59s)
