# todo-cli

A simple command-line todo list app built with Rust. Data disimpan secara lokal dalam format JSON sehingga tetap tersimpan setelah program ditutup.

---

## Requirements

- Rust & Cargo (rustc 1.70+)
- Tidak ada dependency sistem tambahan

Cek instalasi:

```bash
rustc --version
cargo --version
```

---

## Instalasi

Clone atau buat project:

```bash
cargo new todo-cli
cd todo-cli
```

Build project:

```bash
cargo build --release
```

Binary tersedia di `target/release/todo-cli`.

Opsional — copy ke PATH supaya bisa dipanggil dari mana saja:

```bash
cp target/release/todo-cli ~/.local/bin/todo
```

---

## Penggunaan

```
todo list              Tampilkan semua todo
todo add <judul>       Tambah todo baru
todo done <id>         Tandai todo sebagai selesai
todo delete <id>       Hapus todo berdasarkan ID
todo clear             Hapus semua todo
```

Selama development, gunakan `cargo run --` sebagai pengganti nama binary:

```bash
cargo run -- list
cargo run -- add Belajar Rust ownership
cargo run -- done 1
cargo run -- delete 2
cargo run -- clear
```

---

## Contoh Output

```
Daftar Todo:
----------------------------------------
[x] 1 - Belajar Rust ownership
[ ] 2 - Buat project CLI pertama
[ ] 3 - Pahami konsep borrowing
----------------------------------------
1/3 selesai
```

---

## Struktur Project

```
todo-cli/
├── Cargo.toml
├── Cargo.lock
├── todos.json          <- dibuat otomatis saat pertama kali add todo
└── src/
    ├── main.rs         <- entry point & routing command
    ├── todo.rs         <- struct Todo & method display
    └── storage.rs      <- baca/tulis data ke file JSON
```

---

## Dependencies

| Crate | Versi | Kegunaan |
|---|---|---|
| serde | 1.0 | Serialisasi/deserialisasi struct |
| serde_json | 1.0 | Format penyimpanan data (JSON) |

---

## Data Storage

Todo disimpan di file `todos.json` pada direktori tempat binary dijalankan. Format file:

```json
[
  {
    "id": 1,
    "title": "Belajar Rust ownership",
    "completed": true
  },
  {
    "id": 2,
    "title": "Buat project CLI pertama",
    "completed": false
  }
]
```

File ini dibuat otomatis saat pertama kali menambahkan todo. Tidak perlu dibuat manual.

---

## License

MIT
