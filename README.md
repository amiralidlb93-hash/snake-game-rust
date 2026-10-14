Rust Snake

A lightweight terminal Snake game written in Rust and crossterm. Guide the snake, collect fruit, score points, and watch the game speed up as your score grows.

یک بازی Snake ترمینالی و سبک، نوشته‌شده با Rust و crossterm. مار را هدایت کن، میوه‌ها را جمع کن، امتیاز بگیر و با بالا رفتن امتیاز سرعت بازی را بیشتر کن.

Features | ویژگی‌ها

•
Arrow keys or W A S D to move | حرکت با کلیدهای جهت‌دار یا W A S D

•
Pause/resume with P or Space | مکث و ادامه با P یا فاصله

•
Restart with R; quit with Q or Esc | شروع دوباره با R و خروج با Q یا Esc

•
Increasing speed as you score | افزایش سرعت با بالا رفتن امتیاز

•
Colored terminal rendering | نمایش رنگی در ترمینال

•
Wall and self-collision handling | تشخیص برخورد با دیوار و بدن مار

•
Unit tests for movement, growth, and collisions | آزمون واحد برای حرکت، رشد و برخورد

•
One direct dependency: crossterm | تنها وابستگی مستقیم: crossterm

Requirements | پیش‌نیازها

•
Rust and Cargo; Rust 1.75 or newer is recommended.
Rust و Cargo؛ نسخهٔ Rust 1.75 یا جدیدتر پیشنهاد می‌شود.

•
A UTF-8 terminal at least about 62×22 characters.
ترمینال UTF-8 با ابعاد حداقل حدود 62×22 نویسه.

Run | اجرا

Bash


git clone <repository-url>
cd rust-snake
cargo run --release



If you downloaded the ZIP, extract it and run cargo run --release from the directory containing Cargo.toml.
اگر فایل ZIP را دانلود کرده‌ای، آن را استخراج کن و فرمان را از پوشه‌ای که Cargo.toml در آن است اجرا کن.

Controls | کنترل‌ها

Key
Action
کلید
عملکرد
Arrows / W A S D
Move
جهت‌ها / W A S D
حرکت
P / Space
Pause or resume
P / فاصله
مکث یا ادامه
R
Restart
R
شروع دوباره
Q / Esc
Quit
Q / Esc
خروج




Test and build | تست و ساخت

Bash


cargo test
cargo build --release



The release executable is created at target/release/rust-snake.
فایل اجرایی نسخهٔ بهینه‌شده در target/release/rust-snake ساخته می‌شود.

Project structure | ساختار پروژه

Plain Text


rust-snake/
├── Cargo.toml
├── Cargo.lock
├── LICENSE
├── README.md
├── REPOSITORY-METADATA.md
└── src/
    └── main.rs



License | مجوز

This project is licensed under the MIT License. See LICENSE for the full text.
این پروژه تحت مجوز MIT منتشر می‌شود. متن کامل مجوز در فایل LICENSE قرار دارد.

