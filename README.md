# Image Server

## 📌 Overview

The **Image Server** is a high-performance Rust-based web server designed for serving and managing images efficiently. It provides robust API endpoints to handle image uploads, retrieval, and manipulation with minimal latency.

## 🚀 Features

- 📷 **Fast image uploading and retrieval**
- 🖼️ **Supports multiple image formats (JPEG, PNG, etc.)**
- 🔄 **Image resizing and processing**
- ⚡ **Optimized for performance using Rust**
- 🔐 **Secure endpoints with authentication**
- 📁 **Easy integration with front-end applications**

## 🛠️ Installation

### Prerequisites

Ensure you have the following installed:

- [Rust](https://www.rust-lang.org/tools/install)
- [Cargo](https://doc.rust-lang.org/cargo/)

### Steps to Install

```sh
git clone https://github.com/Merthoshan/image_server.git
cd image_server
cargo build --release
```

## 🔧 Usage

### Run the Server

```sh
cargo run --release
```

The server will start and listen on `http://localhost:8080` by default.

### API Endpoints

Example using `curl`:

```sh
curl -X POST --data-binary @"test.jpeg" "http://127.0.0.1:3000/contrast?value=10.0" --output contrast.jpeg
```

This command:
- Uses `http://127.0.0.1:3000` as the local server address and port.
- Takes in `test.jpeg` as the input image.
- Sends the request to the `/contrast` route with a query parameter `value=10.0`.
- Outputs the processed image as `contrast.jpeg`.

## 📜 License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## 🤝 Contributing

Contributions are welcome! Please follow these steps:

1. Fork the repository
2. Create a new branch (`git checkout -b feature-xyz`)
3. Commit your changes (`git commit -m 'Add feature xyz'`)
4. Push to the branch (`git push origin feature-xyz`)
5. Open a Pull Request

## 📧 Contact

For any issues or inquiries, please open an issue in the repository or reach out via email.

