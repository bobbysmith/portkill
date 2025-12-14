# portkill

A CLI utility to terminate processes listening on a TCP port.

`portkill` finds and stops a process that is holding onto a port without needing to manually inspect process lists or copy PIDs.

---

## Why?

When a port is already in use, you usually need to look up the process and kill it manually.

`portkill` reduces that to a single command:

```bash
portkill 3000
```

---

## Installation

### Homebrew (macOS & Linux)

```bash
brew tap YOURNAME/portkill
brew install portkill
```

---

### Prebuilt binaries

Prebuilt binaries are available on the
GitHub Releases page: https://github.com/YOURNAME/portkill/releases

Download the archive for your platform, extract it, and place `portkill` somewhere in your `$PATH`.

Example (macOS):

```bash
curl -L https://github.com/YOURNAME/portkill/releases/latest/download/portkill-*-apple-darwin.zip -o portkill.zip
unzip portkill.zip
chmod +x portkill
mv portkill /usr/local/bin/
```

---

### From source (Cargo)

```bash
cargo install portkill
```

This installs `portkill` into `~/.cargo/bin`.

Make sure `~/.cargo/bin` is included in your `$PATH`.

---

## Updating

### Homebrew
If you installed via Homebrew updates are handled automatically:
```bash
    brew update
    brew upgrade portkill
```

### Prebuilt binaries
Download the latest release for your platform and replace the existing binary in your PATH.

### Cargo
Reinstall using:
```bash
    cargo install portkill --force
```

---

## Usage

```bash
portkill <port>
```

### Examples

```bash
$ portkill 3000
killed port 3000 (node at pid 48291)
```

```bash
$ portkill 8000
nothing running on port 8000
```

```bash
$ portkill 80
found nginx on port 80 (pid 123) but could not kill it
```

Note: killing processes you do not own (like services bound to privileged ports) may fail unless `portkill` is run with sufficient permissions.


---

## Platform support

- macOS (Intel & Apple Silicon)
- Linux (glibc)

---

## License

MIT
