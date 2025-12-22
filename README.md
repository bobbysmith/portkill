portkill
===
A CLI utility to terminate processes listening on a TCP port.
`portkill` finds and stops a process that is holding onto a port without needing to manually inspect process lists or copy PIDs.

---

### Why?
When a port is already in use you usually need to look up the process, copy the pid, and then kill it. This kills it dead with none of the legwork.
`portkill` reduces that to a single command:

```
portkill 3000
```

---

### Installation
#### Homebrew (macOS & Linux)
```
brew tap bobbysmith/portkill
brew install portkill
```

#### Prebuilt binaries
Prebuilt binaries are available at: https://github.com/bobbysmith/portkill/releases
Download the archive for your platform, extract it, and place `portkill` somewhere in your `$PATH`.

Example (macOS):
```
curl -L https://github.com/bobbysmith/portkill/releases/latest/download/portkill-*-apple-darwin.zip -o portkill.zip
unzip portkill.zip
chmod +x portkill
mv portkill /usr/local/bin/
```

#### From source (Cargo)
```
cargo install portkill
```

This installs `portkill` into `~/.cargo/bin`.
Make sure `~/.cargo/bin` is included in your `$PATH`.

---

### Updating
#### Homebrew
```
brew update
brew upgrade portkill
```

#### Prebuilt binaries
Download the latest release for your platform and replace the existing binary in your PATH.

#### Cargo
```
cargo install portkill --force
```

---

### Usage
```
portkill <port>
```

#### Examples
```
$ portkill 3000
[killed] Python (pid 123) on port 3000
```

```
$ portkill 8000
no processes found on port 8000
```

```
$ portkill 80
[error] failed to kill nginx (pid 123) on port 80
```

Note: killing processes you do not own (like services bound to privileged ports) may fail unless `portkill` is run with sufficient permissions.

---

### Platform support
- macOS (Intel & Apple Silicon)
- Linux (glibc)

---

### License
MIT
