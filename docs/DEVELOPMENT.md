# 开发指南

本文档介绍如何设置开发环境并开始开发 Alias Assistant。

## 前置要求

- [Node.js](https://nodejs.org/) (推荐使用 LTS 版本)
- [pnpm](https://pnpm.io/)
- [Rust](https://www.rust-lang.org/) (Tauri 会自动安装，如需手动安装请参考 [Tauri 文档](https://v2.tauri.app/guides/getting-started/prerequisites))

## 安装依赖

```bash
pnpm install
```

## 开发模式

```bash
pnpm tauri dev
```

## 构建应用

```bash
# 构建所有平台的安装包
pnpm run tauri:build

# 仅构建 macOS App Bundle 和 DMG
pnpm run tauri:build:macos

# 仅构建 macOS App Bundle
pnpm run tauri:build:macos:app
```

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
│   ├── composables/               # Vue Composables
│   │   └── useUpdater.ts          # 更新检查逻辑
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
│   ├── generate-icons.js           # 图标生成脚本
│   ├── generate-updater-keypair.sh # 更新密钥对生成脚本
│   └── type-check.sh               # 类型检查脚本
├── docs/                           # 文档
│   ├── DEVELOPMENT.md              # 开发指南（本文件）
│   └── RELEASE.md                  # 发布指南
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

