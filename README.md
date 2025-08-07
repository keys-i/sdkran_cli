### **SDKRAN CLI** - A Fast SDK Manager

**SDKRAN CLI** is a command-line tool for managing SDKs, providing faster installation, version switching, and minimal resource usage. It’s designed to be an efficient, lightweight alternative to SDKMAN, working on macOS, Linux, and Windows.

---

### **Project Structure**

```
sdkran_cli/
├── src/
│   └── main.rs        # Main entry point for the program
├── Cargo.toml         # Dependencies and project settings
└── README.md          # Documentation
```

---

### **Dependencies**

In the `Cargo.toml` file, you'll have the following:

```toml
[package]
name = "sdkran_cli"
version = "0.1.0"
edition = "2021"

[dependencies]
clap = "4.0"  # For handling command-line arguments
```

---

### **How to Run SDKRAN CLI**

1. **Clone the repository**:

   ```bash
   git clone https://github.com/your-username/sdkran_cli.git
   cd sdkran_cli
   ```

2. **Build the project**:

   Install dependencies and build the project:

   ```bash
   cargo build --release
   ```

3. **Run the CLI**:

   Run the CLI using:

   ```bash
   cargo run
   ```

   To test the basic functionality, you can add a simple command like `install` (this is just an example):

   ```bash
   cargo run -- install java
   ```

---

### **Basic Functionality**

1. **Install SDK**:

   A simple command to install an SDK (e.g., `java`):

   ```bash
   cargo run -- install java
   ```
