mod chunker;
use chunker::chunk_text;

fn main() {
    let text = "Hello, world! This is a test. How are you? I'm doing well, thank you for asking. I'm glad to be here. Let's get started. First, we need to install the Rust programming language. To do that, we need to install the Rust toolchain. The Rust toolchain includes the Rust compiler, package manager, and other tools we'll need to build Rust projects. We'll also need to set up our development environment. Once we have the toolchain installed, we can use it to build and run Rust programs. Let's start by installing the Rust toolchain. Open your terminal and run the following command:";
    let chunks = chunk_text(text, 10, 2).unwrap();

    println!("{:?}", chunks);
}
