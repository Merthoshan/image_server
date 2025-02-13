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

The server will start and listen on `http://localhost:3000` by default.

### API Endpoints

| Route            | Method | Parameters                   | Description                           |
|-----------------|--------|-----------------------------|---------------------------------------|
| `/grayscale`    | POST   | None                        | Converts the image to grayscale.      |
| `/invert`       | POST   | None                        | Inverts the colors of the image.      |
| `/rotate90`     | POST   | None                        | Rotates the image 90 degrees.         |
| `/blur`         | POST   | `sigma` (float)             | Applies a blur effect with intensity. |
| `/brightness`   | POST   | `value` (int)               | Adjusts the brightness of the image.  |
| `/contrast`     | POST   | `value` (float)             | Adjusts the contrast of the image.    |

### Example Usage

#### Convert Image to Grayscale

```sh
curl -X POST --data-binary @"test.jpeg" "http://127.0.0.1:3000/grayscale" --output grayscale.jpeg
```

#### Apply Blur Effect

```sh
curl -X POST --data-binary @"test.jpeg" "http://127.0.0.1:3000/blur?sigma=2.0" --output blurred.jpeg
```

#### Adjust Brightness

```sh
curl -X POST --data-binary @"test.jpeg" "http://127.0.0.1:3000/brightness?value=10" --output brightened.jpeg
```

#### Adjust Contrast

```sh
curl -X POST --data-binary @"test.jpeg" "http://127.0.0.1:3000/contrast?value=1.5" --output contrast.jpeg
```

#### Rotate Image by 90 Degrees

```sh
curl -X POST --data-binary @"test.jpeg" "http://127.0.0.1:3000/rotate90" --output rotated.jpeg
```

## 🛠 Configuration

You can modify the `config.toml` file to change settings like the server port and storage directory.

Example:

```toml
[server]
port = 3000
storage_path = "./uploads"
```

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

