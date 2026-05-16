# ⚡ QR Generator — Built in Rust

> Blazing-fast QR code generator built with **Rust** — the language used by Microsoft, Discord & Linux Kernel.
> Generate PNG and SVG QR codes directly from your terminal.

---

## 🪟 Windows Setup

### Step 1 — Install Rust
Download and run the installer from:
👉 https://rustup.rs

### Step 2 — Install MinGW64 (Windows Linker)
Download from:
👉 https://www.mingw-w64.org

After installing, add to PATH:
```bash
set PATH=C:\mingw64\bin;%PATH%
```

### Step 3 — Set GNU Toolchain
```bash
rustup toolchain install stable-x86_64-pc-windows-gnu
rustup default stable-x86_64-pc-windows-gnu
```

### Step 4 — Clone & Run
```bash
git clone https://github.com/hemil-59s/qr-generator
cd qr-generator
cargo run -- --text "https://yourlink.com" --output qr.png
```

---

## 🐧 Linux / Mac Setup

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
git clone https://github.com/hemil-59s/qr-generator
cd qr-generator
cargo run -- --text "https://yourlink.com" --output qr.png
```

---

## 🚀 Usage Examples

```bash
# Any URL
cargo run -- --text "https://yourlink.com" --output qr.png

# SVG format
cargo run -- --text "https://yourlink.com" --output qr.svg

# WhatsApp link
cargo run -- --text "https://wa.me/919876543210" --output whatsapp.png

# Plain text
cargo run -- --text "Hello World" --output hello.png
```

---

## ⚙️ Options

| Flag | Required | Description |
|------|---------|-------------|
| `--text` | ✅ Yes | URL or text to encode |
| `--output` | ❌ No (default: qr.png) | Output filename (.png or .svg) |

---

## 🛠️ Built With

- **Rust** — Systems programming language
- **qrcode-rs** — QR encoding
- **image-rs** — Image processing

---

## 📄 License

MIT © [hemil-59s](https://github.com/hemil-59s)
