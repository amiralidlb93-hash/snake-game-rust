# Snake Game Rust

**Play a classic Snake game in your browser. The game engine is written in Rust and compiled to WebAssembly; the page uses a lightweight HTML/CSS/JavaScript shell for rendering and controls.**

نسخهٔ مرورگری بازی مار را اجرا کن. موتور بازی با Rust نوشته و به WebAssembly کامپایل می‌شود؛ رابط صفحه با HTML/CSS/JavaScript نمایش و کنترل‌ها را مدیریت می‌کند.

## Play online | بازی آنلاین

[Play Snake Game Rust](https://amiralidlb93-hash.github.io/snake-game-rust/)

## Features | امکانات

- Rust game logic compiled to WebAssembly | منطق بازی Rust کامپایل‌شده به WebAssembly
- Browser-based gameplay; no installation or account | اجرای بازی در مرورگر، بدون نصب و حساب کاربری
- Keyboard and touch controls | کنترل با صفحه‌کلید و دکمه‌های لمسی
- Pause, restart, increasing speed, and locally stored best score | مکث، شروع دوباره، افزایش سرعت و ذخیرهٔ رکورد در مرورگر
- Original terminal edition remains available | نسخهٔ ترمینالی اصلی نیز موجود است

## Browser controls | کنترل نسخهٔ مرورگری

| Key | Action | کلید | عملکرد |
| --- | --- | --- | --- |
| Arrow keys / `W A S D` | Move | کلیدهای جهت‌دار / `W A S D` | حرکت |
| `Space` | Pause / resume | فاصله | مکث / ادامه |
| `R` | Restart | `R` | شروع دوباره |

## Run the terminal edition | اجرای نسخهٔ ترمینالی

Install Rust and Cargo, then run:

```bash
cargo run --release
```

Rust و Cargo را نصب کن و از پوشهٔ مخزن اجرا کن:

```bash
cargo run --release
```

## Build the WebAssembly game | ساخت نسخهٔ WebAssembly

```bash
rustup target add wasm32-unknown-unknown
cargo build --release --manifest-path wasm/Cargo.toml --target wasm32-unknown-unknown
cp wasm/target/wasm32-unknown-unknown/release/snake_engine.wasm ./snake_engine.wasm
```

The generated `snake_engine.wasm` belongs beside `index.html`; keep the `assets/` folder alongside it.  
فایل `snake_engine.wasm` ساخته‌شده باید کنار `index.html` باشد و پوشهٔ `assets/` نیز در ریشه بماند.

## GitHub Pages | انتشار در GitHub Pages

The site entry point is the root-level `index.html`. Open **Settings → Pages → Build and deployment**, choose **Deploy from a branch**, select `main` and `/(root)`, then save. Once Pages finishes deploying, the URL above will open the game instead of displaying the repository README.

صفحهٔ سایت، فایل `index.html` در ریشهٔ مخزن است. در **Settings → Pages → Build and deployment**، گزینهٔ **Deploy from a branch** را انتخاب کن، شاخهٔ `main` و مسیر `/(root)` را تنظیم و ذخیره کن. پس از انتشار Pages، نشانی بالا خود بازی را نشان می‌دهد، نه README مخزن را.

## License | مجوز

MIT License — see [`LICENSE`](LICENSE).  
مجوز MIT — متن کامل در فایل [`LICENSE`](LICENSE) است.
