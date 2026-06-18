---
title: "Simple REST API With Actix-Web"
author: "Peter Nyando"
date: "March 20, 2025"
description: "Actix Web lets you quickly and confidently develop web services in Rust and this guide will get you going in no time"
image: "../assets/img/crab.png"
---

### Introduction

> Actix Web lets you quickly and confidently develop web services in Rust and this guide will get you going in no time

Actix-Web is a powerful, fast, and flexible Rust web framework designed for building high-performance applications. In this guide, we'll walk through how to build a simple REST API with Actix-Web, covering everything from setting up the project to handling requests and responses

### Setting Up the Project

First, ensure you have Rust installed. If not, install it using [Rustup](https://rustup.rs/):
Next, create a new Actix-Web project:
Add the necessary dependencies in **Cargo.toml**

### Hello, world!

1. Start by creating a new binary-based Cargo project and changing into the new directory:

```sh
cargo new hello-world
cd hello-world
```

2. Add actix-web as a dependency of your project by adding the following to your **Cargo.toml** file.

```toml
[dependencies]
actix-web = "4"
serde = { version = "1", features = ["derive"] }
serde_json = "1"

```

Request handlers use async functions that accept zero or more parameters. These parameters can be extracted from a request (see FromRequest trait) and returns a type that can be converted into an HttpResponse (see Responder trait):

3. Replace the contents of src/main.rs with the following:

```rust
use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
use serde::{Deserialize, Serialize};

#[derive(Serialize)]
struct Message {
    message: String,
}

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().json(Message {
        message: "Hello, world!".to_string(),
    })
}

#[derive(Deserialize)]
struct EchoRequest {
    text: String,
}

#[post("/echo")]
async fn echo(req_body: web::Json<EchoRequest>) -> impl Responder {
    HttpResponse::Ok().json(Message {
        message: req_body.text.clone(),
    })
}

async fn manual_hello() -> impl Responder {
    HttpResponse::Ok().json(Message {
        message: "Hey there!".to_string(),
    })
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(hello)
            .service(echo)
            .route("/hey", web::get().to(manual_hello))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}

```

### Build and Run the Server

Run the following command to compile and start your server:

```sh
cargo run
```

### Access the Server

Once the server is running, you should see output like:

1. GET request:

```sh
curl http://127.0.0.1:8080/
```

Expected response:

```sh
{"message":"Hello, world!"}
```

2. POST request (JSON Echo):

```sh
curl -X POST http://127.0.0.1:8080/echo -H "Content-Type: application/json" -d '{"text": "Hello Actix"}'

```

Expected response:

```sh
{"message":"Hello Actix"}
```

### Stop the Server

Press Ctrl + C in the terminal to stop the server.


### Conclusion

Actix-Web provides a powerful and efficient way to build REST APIs in Rust. This guide covered the basics of setting up an Actix-Web server and implementing simple Read operations. Next steps could include full CRUD operations and integrating a database like PostgreSQL or adding authentication.
