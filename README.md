# 📝 Aplikasi Todo List CRUD Sederhana dengan Axum & PostgreSQL

> **Proyek pembelajaran sederhana untuk memahami framework web Axum di Rust, dengan menerapkan pola CRUD menggunakan PostgreSQL sebagai database.**

Aplikasi ini dibangun dengan tujuan utama: **belajar Axum dari dasar**. Dengan topik yang sangat umum — yaitu Todo List — kamu bisa fokus pada bagaimana cara:

- Menangani HTTP request & response
- Menghubungkan ke database PostgreSQL
- Menulis query SQL yang aman dan efisien
- Mengelola state dan routing di Axum
- Mengembalikan error dengan kode status HTTP yang tepat

---

## 🎯 Tujuan Pembelajaran

Proyek ini sengaja dibuat **minimalis dan flat** — tanpa arsitektur kompleks, tanpa layer service/repository, tanpa dependency injection — agar kamu bisa:

✅ Fokus pada **cara kerja Axum** tanpa distraksi  
✅ Memahami **alur data dari request → database → response**  
✅ Belajar **SQLx** — library database async yang powerful dan type-safe  
✅ Menerapkan **CRUD lengkap** (Create, Read, Update, Delete) dalam konteks nyata  
✅ Menggunakan **PostgreSQL dengan UUID dan timestamp**  
✅ Mengelola **koneksi database sebagai state global** di Axum  
✅ Mengirim dan menerima **JSON dengan serde**

---

## 🏗️ Arsitektur Proyek

Arsitektur proyek ini disengaja **sesederhana mungkin**:

- Semua logika handler, routing, dan model diletakkan di **satu file utama: `main.rs`** — untuk memudahkan pemahaman dan pelacakan alur.
- Tidak ada pemisahan modul yang rumit — cocok untuk pemula yang ingin melihat “semua dalam satu tempat”.
- Koneksi database dikelola secara **singleton** dan disuntikkan ke handler menggunakan `State` dari Axum.
- Konfigurasi database cukup dengan satu file `.env` — tidak ada sistem config yang berbelit.

> 💡 Proyek ini bukan untuk produksi — tapi **sangat ideal untuk pembelajaran**. Setelah paham dasar, kamu bisa bereksperimen memecah kode, menambahkan validasi, logging, testing, atau autentikasi.

---

## 🛠️ Teknologi yang Digunakan

- **Bahasa Pemrograman**: Rust (versi stabil)
- **Framework Web**: [Axum](https://github.com/tokio-rs/axum) — modern, ergonomic, dibangun di atas Tokio & Tower
- **Database**: PostgreSQL — open-source RDBMS andalan
- **ORM/Query Builder**: [SQLx](https://github.com/launchbadge/sqlx) — async, compile-time checked, zero-dsl
- **Serialization**: [Serde](https://serde.rs/) — untuk konversi JSON ↔ struct
- **ID Unik**: [uuid](https://github.com/uuid-rs/uuid) — untuk primary key
- **Datetime**: [chrono](https://github.com/chronotope/chrono) — untuk timestamp
- **Dotenv**: [dotenvy](https://github.com/dotenv-rs/dotenvy) — untuk baca file `.env`
- **Runtime Async**: [Tokio](https://tokio.rs/) — untuk menjalankan server

---

## 💡 Filosofi Proyek Ini

> “Jangan belajar arsitektur dulu — belajar dulu bagaimana semuanya bekerja.”

Banyak tutorial langsung membagi kode ke dalam `handlers/`, `models/`, `services/`, dll — yang justru membuat pemula bingung: “Dari mana mulainya?”

Di sini, **semua ada di satu tempat** — sehingga kamu bisa:

- Baca dari atas ke bawah
- Lihat bagaimana request masuk → diproses → query ke DB → kembali ke client
- Modifikasi dengan mudah
- Bereksperimen tanpa takut merusak struktur

Setelah kamu paham dasar-dasarnya, barulah kamu bisa refactor ke arsitektur yang lebih scalable.

---

## 📚 Untuk Siapa Proyek Ini?

- Pemula Rust yang ingin belajar membangun backend
- Developer yang ingin mencoba Axum tanpa boilerplate
- Siapa saja yang ingin contoh implementasi CRUD nyata dengan PostgreSQL
- Kamu yang ingin memahami cara kerja web framework di Rust — tanpa magic

---

## 🧩 Apa yang Tidak Ada (Sengaja!)

- ❌ Tidak ada MVC
- ❌ Tidak ada dependency injection
- ❌ Tidak ada middleware kompleks
- ❌ Tidak ada auth/JWT
- ❌ Tidak ada logging terstruktur (selain `env_logger`)
- ❌ Tidak ada testing otomatis
- ❌ Tidak ada Docker

> Semua itu bisa kamu tambahkan **setelah kamu paham dasar-dasarnya** — dan itu bagian dari proses belajar!

---

## 📜 Lisensi

MIT — bebas dipelajari, dimodifikasi, dan digunakan untuk apa pun — termasuk proyek pribadi atau komersial.

---

## 💬 Ingin Kontribusi atau Punya Pertanyaan?

Silakan buka issue atau PR! Proyek pembelajaran jauh lebih menyenangkan jika dilakukan bersama 😊

--

# 📖 Penjelasan Logika `main.rs`

File `main.rs` ini adalah **jantung dari aplikasi Todo List CRUD** kamu. Semua logika — mulai dari routing, handler, koneksi database, hingga struktur data — diletakkan di sini secara sengaja agar mudah dipelajari.

---

## 🧩 1. Import & Setup Awal

```rust
mod db;
use axum::{routing::get, Router, Json, extract::{Path, State}};
use tokio::net::TcpListener;
use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use std::net::SocketAddr;
use uuid::Uuid;
use chrono::{DateTime, Utc};
```

- `mod db;` → modul untuk koneksi ke database (biasanya ada di `db.rs`)
- `axum` → framework web yang kamu pakai
- `tokio` → runtime async untuk menjalankan server
- `serde` → untuk serialisasi/deserialisasi JSON
- `sqlx` → untuk berkomunikasi dengan PostgreSQL
- `uuid` → untuk ID unik todo
- `chrono` → untuk timestamp `created_at`

---

## 🧾 2. Struktur Data (Struct)

### `Todo` — representasi data todo di database

```rust
#[derive(Serialize, Deserialize, Debug)]
struct Todo {
    id: Uuid,
    title: String,
    completed: bool,
    created_at: DateTime<Utc>,
}
```

➡️ Ini adalah bentuk data yang akan dikirim ke/dari client dalam bentuk JSON.  
➡️ Juga dipakai untuk menerima hasil query dari database (`query_as!`).

---

### `CreateTodo` — data yang dikirim saat membuat todo baru

```rust
#[derive(Deserialize, Debug)]
struct CreateTodo {
    title: String,
    completed: Option<bool>,
}
```

➡️ Hanya `title` yang wajib.  
➡️ `completed` opsional — jika tidak dikirim, default-nya `false`.

---

### `UpdateTodo` — data yang dikirim saat update todo

```rust
#[derive(Deserialize, Debug)]
struct UpdateTodo {
    title: Option<String>,
    completed: Option<bool>,
}
```

➡️ Kedua field opsional — artinya kamu bisa update hanya `title`, hanya `completed`, atau keduanya.  
➡️ Jika tidak dikirim, nilai lama tidak berubah (pakai `COALESCE` di SQL).

---

## 🧭 3. Handler (Fungsi Endpoint)

Setiap handler menerima request, berbicara dengan database, lalu mengembalikan response.

---

### ➕ `create_todo` — membuat todo baru

```rust
async fn create_todo(
    State(pool): State<PgPool>,
    Json(payload): Json<CreateTodo>,
) -> Result<Json<Todo>, (StatusCode, String)> {
    let completed = payload.completed.unwrap_or(false);
    let todo = sqlx::query_as!(...)
        .fetch_one(&pool)
        .await?;
    Ok(Json(todo))
}
```

➡️ Ambil `title` dan `completed` dari body JSON.  
➡️ Jika `completed` tidak dikirim → set ke `false`.  
➡️ Simpan ke database, lalu kembalikan data yang baru dibuat.

---

### 📄 `get_todos` — ambil semua todo

```rust
async fn get_todos(State(pool): State<PgPool>) -> Result<Json<Vec<Todo>>, ...> {
    let todos = sqlx::query_as!(...)
        .fetch_all(&pool)
        .await?;
    Ok(Json(todos))
}
```

➡️ Query semua data todo dari tabel `todos`.  
➡️ Kembalikan sebagai array JSON.

---

### 📄 `get_todo` — ambil satu todo berdasarkan ID

```rust
async fn get_todo(
    State(pool): State<PgPool>,
    Path(id): Path<Uuid>,
) -> Result<Json<Todo>, ...> {
    let todo = sqlx::query_as!(...)
        .fetch_optional(&pool)
        .await?
        .ok_or(...)?;
    Ok(Json(todo))
}
```

➡️ Ambil `id` dari URL (`/todos/:id`).  
➡️ Cari todo dengan ID tersebut.  
➡️ Jika tidak ditemukan → kembalikan error 404.

---

### ✏️ `update_todo` — update todo berdasarkan ID

```rust
async fn update_todo(
    State(pool): State<PgPool>,
    Path(id): Path<Uuid>,
    Json(payload): Json<UpdateTodo>
) -> Result<Json<Todo>, ...> {
    let todo = sqlx::query_as!(
        ...
        "UPDATE todos SET title = COALESCE($1, title), completed = COALESCE($2, completed) WHERE id = $3 ..."
    )
    .fetch_optional(&pool)
    .await?
    .ok_or(...)?;
    Ok(Json(todo))
}
```

➡️ `COALESCE($1, title)` → jika `$1` (title baru) `NULL`, pakai nilai lama.  
➡️ Sama untuk `completed`.  
➡️ Jadi, kamu bisa kirim hanya field yang ingin diubah — sisanya tetap.

---

### ❌ `delete_todo` — hapus todo berdasarkan ID

```rust
async fn delete_todo(
    State(pool): State<PgPool>,
    Path(id): Path<Uuid>
) -> Result<Json<()>, ...> {
    let result = sqlx::query!("DELETE FROM todos WHERE id = $1", id)
        .execute(&pool)
        .await?;

    if result.rows_affected() == 0 {
        return Err((StatusCode::NOT_FOUND, "..."));
    }

    Ok(Json(())) // 200 OK, body kosong
}
```

➡️ Hapus baris berdasarkan ID.  
➡️ Jika tidak ada baris yang terhapus → artinya ID tidak ditemukan → kembalikan 404.  
➡️ Jika berhasil → kembalikan respons kosong (200 OK).

---

### 🌍 `hello` — endpoint sederhana untuk cek server hidup

```rust
async fn hello() -> &'static str {
    "Hello, World!"
}
```

➡️ Untuk testing: buka `http://localhost:3000` → lihat “Hello, World!”.

---

## 🚀 4. Fungsi `main()` — Menjalankan Server

```rust
#[tokio::main]
async fn main() {
    dotenvy::dotenv().ok();     // muat .env
    env_logger::init();         // aktifkan logging

    let pool = db::connect_db().await.expect("Failed to connect to DB");

    let app = Router::new()
        .route("/", get(hello))
        .route("/todos", get(get_todos).post(create_todo))
        .route("/todos/:id", get(get_todo).put(update_todo).delete(delete_todo))
        .with_state(pool);      // kirim koneksi DB ke semua handler

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    println!("🚀 Listening on http://{}", addr);

    let listener = TcpListener::bind(&addr).await.unwrap();
    axum::serve(listener, app.into_make_service())
        .await
        .unwrap();
}
```

➡️ **Langkah 1**: Muat variabel lingkungan dari `.env` (untuk `DATABASE_URL`).  
➡️ **Langkah 2**: Buat koneksi ke database PostgreSQL.  
➡️ **Langkah 3**: Bangun router Axum:

- `GET /` → hello world
- `GET /todos` → ambil semua todo
- `POST /todos` → buat todo baru
- `GET /todos/:id` → ambil satu todo
- `PUT /todos/:id` → update todo
- `DELETE /todos/:id` → hapus todo

➡️ **Langkah 4**: Bind ke `localhost:3000` dan jalankan server.

---

## 🎯 Kesimpulan

> Semua logika CRUD ada di sini — tanpa arsitektur kompleks.  
> Cocok untuk belajar bagaimana Axum, SQLx, dan async Rust bekerja bersama.

✅ **Create** → `POST /todos`  
✅ **Read** → `GET /todos` dan `GET /todos/:id`  
✅ **Update** → `PUT /todos/:id`  
✅ **Delete** → `DELETE /todos/:id`

---

## 💡 Tips Belajar

- Ubah struct → lihat efeknya di request/response
- Ubah query SQL → lihat bagaimana SQLx membantu dengan type safety
- Tambahkan validasi → misal: title tidak boleh kosong
- Pisahkan handler ke file terpisah → latihan modularisasi
