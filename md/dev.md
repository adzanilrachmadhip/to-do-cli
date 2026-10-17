# Rust CLI Todo List — Step by Step Guide

> **Stack:** Rust + Cargo | **Platform:** CLI | **Level:** Pemula

---

## Struktur Project Akhir

```
todo-cli/
├── Cargo.toml
├── Cargo.lock
└── src/
    ├── main.rs
    ├── todo.rs
    └── storage.rs
```

---

## Step 1 — Buat Project Baru dengan Cargo

Buka terminal, jalankan:

```bash
cargo new todo-cli
cd todo-cli
```

Cek struktur awal:

```
todo-cli/
├── Cargo.toml
└── src/
    └── main.rs
```

Coba jalankan dulu untuk memastikan setup oke:

```bash
cargo run
```

Output: `Hello, world!`

---

## Step 2 — Tambahkan Dependencies di Cargo.toml

Buka `Cargo.toml`, ubah menjadi:

```toml
[package]
name = "todo-cli"
version = "0.1.0"
edition = "2021"

[dependencies]
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"
```

**Penjelasan:**
- `serde` — library untuk serialisasi/deserialisasi data
- `serde_json` — untuk menyimpan & membaca data todo dalam format JSON

Download dependencies:

```bash
cargo build
```

---

## Step 3 — Buat Struct Todo (src/todo.rs)

Buat file baru `src/todo.rs`:

```rust
use serde::{Deserialize, Serialize};

// Struct untuk merepresentasikan satu item todo
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Todo {
    pub id: u32,
    pub title: String,
    pub completed: bool,
}

impl Todo {
    // Constructor untuk membuat todo baru
    pub fn new(id: u32, title: String) -> Self {
        Todo {
            id,
            title,
            completed: false,
        }
    }

    // Method untuk menampilkan todo dengan format yang rapi
    pub fn display(&self) {
        let status = if self.completed { "[x]" } else { "[ ]" };
        println!("{} {} - {}", status, self.id, self.title);
    }
}
```

---

## Step 4 — Buat Modul Storage (src/storage.rs)

Buat file baru `src/storage.rs` — bertugas menyimpan & membaca data dari file JSON:

```rust
use crate::todo::Todo;
use std::fs;

const FILE_PATH: &str = "todos.json";

// Membaca semua todo dari file JSON
pub fn load_todos() -> Vec<Todo> {
    match fs::read_to_string(FILE_PATH) {
        Ok(content) => serde_json::from_str(&content).unwrap_or_else(|_| vec![]),
        Err(_) => vec![], // Kalau file belum ada, return array kosong
    }
}

// Menyimpan semua todo ke file JSON
pub fn save_todos(todos: &Vec<Todo>) {
    let json = serde_json::to_string_pretty(todos).expect("Gagal serialize JSON");
    fs::write(FILE_PATH, json).expect("Gagal menyimpan file");
}
```

---

## Step 5 — Tulis Logic Utama (src/main.rs)

Ubah `src/main.rs` menjadi:

```rust
mod todo;
mod storage;

use std::env;
use todo::Todo;
use storage::{load_todos, save_todos};

fn main() {
    // Ambil argument dari command line
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        print_help();
        return;
    }

    let command = &args[1];

    match command.as_str() {
        "list"   => cmd_list(),
        "add"    => cmd_add(&args),
        "done"   => cmd_done(&args),
        "delete" => cmd_delete(&args),
        "clear"  => cmd_clear(),
        _ => {
            println!("Perintah '{}' tidak dikenal.", command);
            print_help();
        }
    }
}

// ── COMMAND: list ────────────────────────────────────────────────
fn cmd_list() {
    let todos = load_todos();

    if todos.is_empty() {
        println!("Belum ada todo. Tambahkan dengan: todo add <judul>");
        return;
    }

    println!("\nDaftar Todo:");
    println!("{}", "-".repeat(40));
    for todo in &todos {
        todo.display();
    }
    println!("{}", "-".repeat(40));

    let done_count = todos.iter().filter(|t| t.completed).count();
    println!("{}/{} selesai\n", done_count, todos.len());
}

// ── COMMAND: add ─────────────────────────────────────────────────
fn cmd_add(args: &[String]) {
    if args.len() < 3 {
        println!("Penggunaan: todo add <judul todo>");
        return;
    }

    let mut todos = load_todos();
    let title = args[2..].join(" "); // Gabung semua kata setelah 'add'
    let id = todos.iter().map(|t| t.id).max().unwrap_or(0) + 1;

    let new_todo = Todo::new(id, title.clone());
    todos.push(new_todo);
    save_todos(&todos);

    println!("Todo ditambahkan: \"{}\" (ID: {})", title, id);
}

// ── COMMAND: done ─────────────────────────────────────────────────
fn cmd_done(args: &[String]) {
    if args.len() < 3 {
        println!("Penggunaan: todo done <id>");
        return;
    }

    let id: u32 = match args[2].parse() {
        Ok(n) => n,
        Err(_) => {
            println!("ID harus berupa angka.");
            return;
        }
    };

    let mut todos = load_todos();
    let mut found = false;

    for todo in todos.iter_mut() {
        if todo.id == id {
            todo.completed = true;
            println!("Todo #{} ditandai selesai: \"{}\"", id, todo.title);
            found = true;
            break;
        }
    }

    if !found {
        println!("Todo dengan ID {} tidak ditemukan.", id);
        return;
    }

    save_todos(&todos);
}

// ── COMMAND: delete ───────────────────────────────────────────────
fn cmd_delete(args: &[String]) {
    if args.len() < 3 {
        println!("Penggunaan: todo delete <id>");
        return;
    }

    let id: u32 = match args[2].parse() {
        Ok(n) => n,
        Err(_) => {
            println!("ID harus berupa angka.");
            return;
        }
    };

    let mut todos = load_todos();
    let before = todos.len();
    todos.retain(|t| t.id != id);

    if todos.len() == before {
        println!("Todo dengan ID {} tidak ditemukan.", id);
        return;
    }

    save_todos(&todos);
    println!("Todo #{} berhasil dihapus.", id);
}

// ── COMMAND: clear ────────────────────────────────────────────────
fn cmd_clear() {
    save_todos(&vec![]);
    println!("Semua todo telah dihapus.");
}

// ── HELPER: print help ────────────────────────────────────────────
fn print_help() {
    println!("\nRust Todo CLI");
    println!("{}", "-".repeat(35));
    println!("Penggunaan:");
    println!("  cargo run -- list              -> Tampilkan semua todo");
    println!("  cargo run -- add <judul>       -> Tambah todo baru");
    println!("  cargo run -- done <id>         -> Tandai todo selesai");
    println!("  cargo run -- delete <id>       -> Hapus todo");
    println!("  cargo run -- clear             -> Hapus semua todo");
    println!();
}
```

---

## Step 6 — Build & Test

Build project:

```bash
cargo build
```

Coba semua command:

```bash
# Lihat daftar (masih kosong)
cargo run -- list

# Tambah beberapa todo
cargo run -- add Belajar Rust ownership
cargo run -- add Buat project CLI pertama
cargo run -- add Setup VS Code dengan rust-analyzer
cargo run -- add Pahami konsep borrowing

# Lihat daftar
cargo run -- list

# Tandai todo nomor 1 selesai
cargo run -- done 1

# Hapus todo nomor 3
cargo run -- delete 3

# Lihat hasil akhir
cargo run -- list

# Hapus semua
cargo run -- clear
```

---

## Step 7 — Build Binary Final (Opsional)

Kalau mau jalankan langsung tanpa `cargo run`:

```bash
cargo build --release
```

Binary akan ada di `target/release/todo-cli`

```bash
# Jalankan langsung
./target/release/todo-cli list
./target/release/todo-cli add Makan siang
```

Copy binary ke folder PATH supaya bisa dipanggil dari mana saja:

```bash
# Linux/macOS
cp target/release/todo-cli ~/.local/bin/todo

# Sekarang bisa dipakai langsung
todo list
todo add Beli kopi
```

---

## Contoh Output

```
Daftar Todo:
----------------------------------------
[x] 1 - Belajar Rust ownership
[ ] 2 - Buat project CLI pertama
[ ] 4 - Pahami konsep borrowing
----------------------------------------
1/3 selesai
```

---

## Konsep Rust yang Dipelajari di Project Ini

| Konsep | Dipakai Di |
|---|---|
| `struct` | `todo.rs` — mendefinisikan data Todo |
| `impl` + method | `Todo::new()`, `Todo::display()` |
| `Vec<T>` | Menyimpan koleksi todo |
| `match` | Routing command di `main.rs` |
| `Option` & `unwrap_or` | Handling file kosong di `storage.rs` |
| `Result` & error handling | `parse()` untuk konversi ID |
| Module system (`mod`) | Memisahkan kode ke beberapa file |
| `derive` macro | `#[derive(Serialize, Deserialize)]` |
| Ownership & borrowing | `&Vec<Todo>`, `iter_mut()` |

---

## Ide Pengembangan Selanjutnya

- Filter — `todo list --pending` / `todo list --done`
- Edit — `todo edit <id> <judul baru>`
- Priority — tambah field `priority: High/Medium/Low`
- Due date — tambah deadline per todo
- Warna terminal — pakai crate `colored` biar output lebih variatif

---

*Selamat mencoba. Konsistensi adalah kunci menguasai Rust.*