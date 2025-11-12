# 🛡️ XAuthenticator

**XAuthenticator** 是一款现代、安全且易用的 **双重身份验证 (2FA)** 管理工具。  
它帮助你集中管理所有的动态验证码（TOTP / HOTP），支持多设备同步、备份与加密存储，确保你的账号安全不再受限于单一设备。

<p align="center">
  <img src="./.github/icon.png" alt="XAuthenticator Logo" width="160" />
</p>

<p align="center">
  <b>轻量、安全、现代的 2FA 管理工具</b><br/>
  在所有设备上安全地管理您的 2FA。
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

## 🧭 简介 | Overview

**XAuthenticator** 是一款跨平台的 **双重身份验证 (2FA)** 管理工具，支持多种验证协议（TOTP/HOTP），并以现代化的界面与安全的本地加密存储，让你的数字身份更加安全。

> English version is available below.  
> 👉 [Jump to English version](./README_English.md)

---

## ✨ 功能特性 | Features

- 🔐 **多协议支持** — 支持 TOTP / HOTP 验证算法
- 💾 **本地加密存储** — 使用 安全加密
- ☁️ **同步支持** — 可选加密云同步 / 本地备份导出
- 🧩 **便捷导入** — 扫码 / 手动添加 / 导入 Google Authenticator 数据
- 🧱 **跨平台运行** — 支持 Windows / macOS / Linux
- 🎨 **现代 UI 设计** — 基于 Tauri + SvelteKit + shadcn-svelte 打造
- 🛡️ **隐私优先** — 离线工作，无任何数据上报

---

## 🚀 快速开始 | Quick Start

### 📦 安装

#### 从源码构建

```bash
# 克隆仓库
git clone https://github.com/fynntang/XAuthenticator.git
cd XAuthenticator

# 安装依赖
pnpm install

# 构建并运行 (Tauri)
pnpm tauri dev
```

#### 或下载已编译版本

前往 [Releases 页面](https://github.com/fynntang/XAuthenticator/releases) 下载适用于你的系统的安装包。

---

### 🧠 技术栈 | Tech Stack

| 模块      | 技术                                                                             |
|---------|--------------------------------------------------------------------------------|
| 前端界面    | [SvelteKit](https://kit.svelte.dev/) + [shadcn-svelte](https://ui.shadcn.com/) |
| 桌面封装    | [Tauri v2](https://tauri.app/)                                                 |
| 终端依赖    | Rust                                                                           |
| 加密算法    | Kdf + Argon2                                                                   |
| 图标 & UI | TailwindCSS + Lucide Icons                                                     |

---

### 🧩 即将推出 | Coming Soon

- 🔄 云端同步插件（OneDrive / Dropbox / WebDAV）
- 🔒 GPG 加密集成
- 🧱 命令行 (CLI) 工具
- 🌐 浏览器扩展 Companion

---

### 🧾 开源协议 | License

本项目基于 [MIT License](./LICENSE) 开源。

You are free to use, modify and distribute this project under the MIT license.

---

### ❤️ 致谢

- [Tauri](https://v2.tauri.app/)
- [SvelteKit](https://svelte.dev/docs/kit/introduction)
- [shadcn-svelte](https://shadcn-svelte.com/)
- [Lucide Icons](https://lucide.dev/icons/)

所有为安全与开源生态做出贡献的开发者们 🙏

---
<p align="center"> <b>🧭 XAuthenticator — 让你的双重验证更安全、更自由。</b> </p>