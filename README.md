# kotaru wallet

CLI Based EVM wallet written in Rust. Built using `alloy-rs`.

Currently only supports Base Sepolia.

## todo

-   [x] wallet add
-   [x] fetching balance
-   [ ] changing rpc
-   [x] sending tx - only basic txns like ether transfer right now
-   [ ] encrypt priv keys
-   [ ] contract deployments using bytecode

## 🚀 Installation

### **MacOS (via Homebrew)**

If you're using macOS, you can install Kotaru easily using Homebrew:

```
brew tap dawksh/kotaru
brew install kotaru
```

To upgrade to the latest version:

```
brew upgrade kotaru
```

To uninstall:

```
brew uninstall kotaru
```

---

### **Linux & Windows (Build from Source)**

If you're on **Linux** or **Windows**, you can build Kotaru from source using Cargo.

#### **1. Install Rust (if not already installed)**

```
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

or follow [Rust installation guide](https://www.rust-lang.org/tools/install).

#### **2. Clone the Repository**

```
git clone https://github.com/dawksh-wallet.git
cd kotaru
```

#### **3. Build the Binary**

```
cargo build --release
```

The compiled binary will be located at:

```
target/release/kotaru
```

#### **4. Install Kotaru**

Move the binary to a directory in your `PATH`, e.g.:

```
sudo mv target/release/kotaru /usr/local/bin/
```

Now, you can run `kotaru` from anywhere!
