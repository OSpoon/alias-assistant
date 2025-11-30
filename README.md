# Alias Assistant ✨

便捷的 macOS（Apple Silicon）shell 别名管理工具，使用 Tauri + Vue + TypeScript 构建。

<div align="center">
  <img src="./screenshot.png" width="40%" />
</div>

## 功能特性

- 🎯 **简洁的界面** - 使用 DaisyUI 和 Tailwind CSS 构建的现代化 UI
- 🔍 **快速搜索** - 实时搜索别名，快速找到你需要的命令
- 📋 **一键复制** - 点击列表项即可复制别名名称到剪贴板
- 🖥️ **快速打开终端** - 点击 Terminal 按钮复制别名并打开新终端窗口
- 🎨 **多主题支持** - 内置 5 款精美主题（Light、Dark、Forest、Synthwave、Dracula）
- 💾 **导入/导出** - 支持导入和导出别名文件，方便备份和迁移
- 🔄 **自动更新** - 内置版本检测和自动更新功能，保持应用最新
- ⚡ **轻量快速** - 基于 Tauri，体积小、性能高
- 🔒 **安全可靠** - 本地存储，数据完全掌控
- 🧩 **组件化架构** - 模块化设计，易于维护和扩展

## 技术栈

- **前端**: Vue 3 + TypeScript + Vite
- **UI 框架**: Tailwind CSS 4 + DaisyUI 5
- **后端**: Rust + Tauri 2
- **包管理**: pnpm

## “Alias Assistant.app”已损坏，无法打开

```bash
# 清除文件的隔离属性
sudo xattr -c /Applications/Alias\ Assistant.app
```

## 开发

### 前置要求

- [Node.js](https://nodejs.org/) (推荐使用 LTS 版本)
- [pnpm](https://pnpm.io/)
- [Rust](https://www.rust-lang.org/) (Tauri 会自动安装，如需手动安装请参考 [Tauri 文档](https://tauri.app/zh-cn/v1/guides/getting-started/prerequisites))

### 安装依赖

```bash
pnpm install
```

### 开发模式

```bash
pnpm tauri dev
```

### 构建应用

```bash
# 构建所有平台的安装包
pnpm run tauri:build

# 仅构建 macOS App Bundle 和 DMG
pnpm run tauri:build:macos

# 仅构建 macOS App Bundle
pnpm run tauri:build:macos:app
```

## 使用说明

### 基本操作

1. **添加别名**：点击右下角的 "+" 按钮，输入别名名称和命令
2. **搜索别名**：在搜索栏输入关键词，实时过滤别名列表
3. **复制别名**：点击列表项任意位置，别名名称会自动复制到剪贴板
4. **打开终端**：点击 "Terminal" 按钮，会复制别名并打开新终端窗口
5. **删除别名**：点击 "Delete" 按钮，确认后删除别名

### 设置功能

点击右上角的设置按钮，可以：

- **切换主题**：选择 5 款内置主题之一，主题会自动保存
- **检查更新**：手动检查应用更新，如有新版本可一键安装
- **导出别名**：将当前所有别名导出为 `.sh` 文件
- **导入别名**：从 `.sh` 文件导入别名（同名别名会被替换）

## 版本管理

项目使用统一的版本号管理，版本号会自动同步到 `package.json`、`tauri.conf.json` 和 `Cargo.toml`。

### 更新版本号

```bash
# 更新补丁版本 (0.1.0 -> 0.1.1)
pnpm run version:patch

# 更新次版本 (0.1.0 -> 0.2.0)
pnpm run version:minor

# 更新主版本 (0.1.0 -> 1.0.0)
pnpm run version:major

# 手动同步版本号
pnpm run version:sync
```

### 生成图标

```bash
# 从项目根目录的 logo.png 生成所有必需的图标格式和尺寸
pnpm run icons:generate
```

### 配置自动更新

应用支持通过 GitHub Releases 自动更新。配置步骤：

1. **生成更新密钥对**：
   ```bash
   ./scripts/generate-updater-keypair.sh
   ```
   或者手动运行：
   ```bash
   pnpm tauri signer generate -w ~/.tauri/myapp.key
   ```

2. **更新配置**：
   - 将生成的公钥添加到 `src-tauri/tauri.conf.json` 的 `plugins.updater.pubkey` 字段
   - 确认 `plugins.updater.endpoints` 中的 GitHub 仓库 URL 正确

3. **构建签名版本**：
   ```bash
   TAURI_PRIVATE_KEY_PATH=~/.tauri/myapp.key pnpm tauri build
   ```

4. **发布更新**：
   - 使用 GitHub Actions 自动构建和发布时，确保使用相同的密钥对签名
   - 确保 `tauri.conf.json` 中设置了 `bundle.createUpdaterArtifacts: true`
   - `tauri-action` 会自动生成更新工件（包括 `.app.tar.gz`、`.sig` 和 `latest.json`）并上传到 GitHub Release
   - `latest.json` 文件会自动上传到每个版本的 Release 中，可通过 `/releases/latest/download/latest.json` 访问
   - 应用会自动从配置的 endpoint 检查更新
   - 如果遇到更新检查失败，请检查：
     - `tauri.conf.json` 中 `bundle.createUpdaterArtifacts` 是否为 `true`
     - 构建时是否正确使用了签名密钥（通过 `TAURI_PRIVATE_KEY_PATH` 环境变量）
     - GitHub Actions 工作流是否成功完成
     - Release 中是否包含 `latest.json` 文件
     - `plugins.updater.endpoints` 配置的 URL 是否正确

## 项目结构

```
alias-assistant/
├── src/                           # Vue 前端代码
│   ├── components/                # Vue 组件
│   │   ├── AddAliasModal.vue     # 添加别名模态框
│   │   ├── AliasItem.vue          # 别名列表项
│   │   ├── AliasList.vue          # 别名列表
│   │   ├── SearchBar.vue           # 搜索栏
│   │   ├── SettingsModal.vue       # 设置模态框
│   │   ├── ThemeSelector.vue      # 主题选择器
│   │   └── ToastNotification.vue  # Toast 通知
│   ├── types/                      # TypeScript 类型定义
│   │   └── alias.ts               # 别名类型
│   ├── App.vue                     # 主应用组件
│   ├── main.ts                     # 入口文件
│   └── index.css                   # 全局样式
├── src-tauri/                      # Tauri 后端代码
│   ├── src/
│   │   ├── main.rs                 # Rust 入口
│   │   └── lib.rs                  # 主要逻辑（别名管理、剪贴板、终端等）
│   ├── capabilities/               # Tauri 权限配置
│   │   └── default.json            # 默认权限
│   ├── icons/                      # 应用图标
│   ├── Cargo.toml                  # Rust 依赖配置
│   └── tauri.conf.json             # Tauri 配置文件
├── scripts/                         # 工具脚本
│   ├── sync-version.js             # 版本号同步脚本
│   └── generate-icons.js           # 图标生成脚本
├── .github/                        # GitHub Actions
│   └── workflows/
│       └── publish.yml              # 自动化构建工作流
└── package.json                    # Node.js 依赖配置
```

## 推荐 IDE 设置

- [VS Code](https://code.visualstudio.com/) + 以下扩展：
  - [Vue - Official](https://marketplace.visualstudio.com/items?itemName=Vue.volar)
  - [Tauri](https://marketplace.visualstudio.com/items?itemName=tauri-apps.tauri-vscode)
  - [rust-analyzer](https://marketplace.visualstudio.com/items?itemName=rust-lang.rust-analyzer)

## 许可证

MIT
