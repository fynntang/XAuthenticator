# 🛡️ Introduction

**XAuthenticator** is a modern, cross-platform **Two-Factor Authentication (2FA)** manager built with performance and
privacy in mind.

It supports TOTP and HOTP tokens, secure local encryption, optional cloud sync, and a clean, minimal interface.


<p align="center">
  <img src="./.github/icon.png" alt="XAuthenticator Logo" width="160" />
</p>

<p align="center">
  <b>Lightweight, secure, and modern 2FA management tools</b><br/>
  Manage your 2FA tokens securely across all your devices.
</p>

<p align="center">
  <a href="https://github.com/fynntang/XAuthenticator/releases">
    <img src="https://img.shields.io/github/v/release/fynntang/XAuthenticator?color=ff4081&label=version&style=for-the-badge" alt="Release Version">
  </a>
  <a href="https://github.com/fynntang/XAuthenticator/actions">
    <img src="https://img.shields.io/github/actions/workflow/status/fynntang/XAuthenticator/build.yml?style=for-the-badge&logo=github" alt="Build Status">
  </a>
  <a href="https://github.com/fynntang/XAuthenticator/blob/main/LICENSE">
    <img src="https://img.shields.io/github/license/fynntang/XAuthenticator?style=for-the-badge" alt="License">
  </a>
  <img src="https://img.shields.io/badge/platform-Windows%20%7C%20macOS%20%7C%20Linux%20%7C%20iOS%20%7C%20Android-blue?style=for-the-badge" alt="Platform">
</p>



---

## ✨ Features

- 🔐 **TOTP & HOTP support** — Compatible with major authenticator apps
- 💾 **Encrypted local storage** — AES-256 + PBKDF2 secure key storage
- ☁️ **Sync & backup** — Encrypted file or cloud synchronization
- 🧩 **Easy import** — QR scanning, manual entry, or import from Google Authenticator
- 🧱 **Cross-platform** — Runs on Windows, macOS, and Linux
- 🎨 **Modern UI** — Built with Tauri + SvelteKit + shadcn-svelte
- 🛡️ **Privacy-first** — Offline by design, no telemetry or tracking

---

## 🚀 Getting Started

### 📦 Installation

#### Built from source

```bash
# Clone the repository
git clone https://github.com/fynntang/XAuthenticator.git
cd XAuthenticator

# Install dependencies
pnpm install

# Run the development server
pnpm tauri dev
```

Or download the latest release from the [Releases page](https://github.com/fynntang/XAuthenticator/releases)

---

### 🧠 Tech Stack

| Component             | Technology                                                                     |
|-----------------------|--------------------------------------------------------------------------------|
| UI                    | [SvelteKit](https://kit.svelte.dev/) + [shadcn-svelte](https://ui.shadcn.com/) |
| Desktop Runtime       | [Tauri v2](https://tauri.app/)                                                 |
| Terminal dependencies | Rust + SQLite                                                                  |
| Crypto                | AES-256  / HMAC-SHA1 / Ed25519                                                 |
| Icons & UI            | TailwindCSS + Lucide Icons                                                     |

---

### 🧩 Upcoming Features

- 🔄 Cloud sync plugins（OneDrive / Dropbox / WebDAV）
- 🔒 GPG encryption support
- 🧱 Command-line (CLI) interface
- 🌐 Web companion extension

---

### 🧾 License

Licensed under the **MIT License** — see [LICENSE](./LICENSE) for details.

---

### ❤️ Thanks

- [Tauri](https://v2.tauri.app/)
- [SvelteKit](https://svelte.dev/docs/kit/introduction)
- [shadcn-svelte](https://shadcn-svelte.com/)
- [Lucide Icons](https://lucide.dev/icons/)

All developers who contribute to the security and open source ecosystem 🙏

--- 
<p align="center"> <b>🧭 XAuthenticator — Secure your identity, everywhere.</b> </p>