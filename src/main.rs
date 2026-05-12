use clap::Parser;
use qrcode::QrCode;
use qrcode::render::svg;
use image::Luma;

/// QR Code Generator
#[derive(Parser)]
#[command(name = "qr-generator", about = "Generate QR codes instantly")]
struct Cli {
    /// Text ya URL jo encode karna hai
    #[arg(short, long)]
    text: String,

    /// Output file (.png ya .svg)
    #[arg(short, long, default_value = "qr.png")]
    output: String,
}

fn main() {
    let cli = Cli::parse();

    let code = QrCode::new(cli.text.as_bytes()).unwrap();

    if cli.output.ends_with(".svg") {
        let svg_str = code.render::<svg::Color>().build();
        std::fs::write(&cli.output, svg_str).unwrap();
    } else {
        let image = code.render::<Luma<u8>>().build();
        image.save(&cli.output).unwrap();
    }

    println!("QR saved → {}", cli.output);
}