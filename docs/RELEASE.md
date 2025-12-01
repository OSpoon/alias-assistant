# 发布指南

本文档介绍如何管理版本号和发布应用。

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

## 自动更新配置

应用支持通过 GitHub Releases 自动更新。

### 首次配置

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
   - 确保 `bundle.createUpdaterArtifacts` 设置为 `true`

3. **配置 GitHub Secrets**：
   - 在 GitHub 仓库的 Settings → Secrets and variables → Actions 中添加：
     - `TAURI_SIGNING_PRIVATE_KEY`：私钥的完整内容
     - `TAURI_SIGNING_PRIVATE_KEY_PASSWORD`：私钥密码（如果私钥有密码保护）

### 构建签名版本

使用 `TAURI_SIGNING_PRIVATE_KEY` 环境变量设置私钥内容：

```bash
TAURI_SIGNING_PRIVATE_KEY="$(cat ~/.tauri/myapp.key)" pnpm tauri build
```

如果私钥有密码保护，还需要设置：

```bash
TAURI_SIGNING_PRIVATE_KEY="$(cat ~/.tauri/myapp.key)"
TAURI_SIGNING_PRIVATE_KEY_PASSWORD="your-password"
pnpm tauri build
```

**注意**：只支持使用 `TAURI_SIGNING_PRIVATE_KEY` 环境变量，不支持 `TAURI_PRIVATE_KEY_PATH`。

### 发布流程

1. **更新版本号**：使用 `pnpm run version:patch/minor/major` 更新版本
2. **提交更改**：提交版本更新到 Git
3. **推送到 release 分支**：GitHub Actions 会自动触发构建和发布
4. **验证发布**：检查 GitHub Releases 页面，确认：
   - 所有构建产物已上传
   - `latest.json` 文件已生成并上传
   - 更新检查功能正常工作

### GitHub Actions 工作流

项目使用 GitHub Actions 自动构建和发布：

- **触发条件**：推送到 `release` 分支或手动触发
- **构建目标**：`aarch64-apple-darwin` (Apple Silicon)
- **自动生成**：`.app.tar.gz`、`.sig` 和 `latest.json` 文件
- **自动上传**：所有文件自动上传到 GitHub Release

详细配置请参考 `.github/workflows/publish.yml`。

