# GitHub Actions 签名密钥配置指南

## 步骤 1: 生成签名密钥对

在本地运行以下命令生成密钥对：

```bash
./scripts/generate-updater-keypair.sh
```

或者手动运行：

```bash
pnpm tauri signer generate -w ~/.tauri/myapp.key
```

## 步骤 2: 获取私钥内容

查看生成的私钥文件内容：

```bash
cat ~/.tauri/myapp.key
```

**重要**: 复制整个文件内容（包括所有行），这将用于下一步。

## 步骤 3: 配置 GitHub Secrets

1. 进入你的 GitHub 仓库
2. 点击 **Settings** → **Secrets and variables** → **Actions**
3. 点击 **New repository secret** 添加以下密钥：

### 必需的 Secret

- **名称**: `TAURI_SIGNING_PRIVATE_KEY`
  - **值**: 粘贴步骤 2 中复制的私钥完整内容
  - **说明**: Tauri 更新器签名私钥

### 可选的 Secret（如果密钥有密码保护）

- **名称**: `TAURI_SIGNING_PRIVATE_KEY_PASSWORD`
  - **值**: 你设置的保护私钥的密码
  - **说明**: 如果生成密钥时设置了密码，需要提供此密码

## 步骤 4: 验证配置

配置完成后，GitHub Actions 工作流会在构建时自动：

1. 从 Secrets 读取私钥
2. 创建 `~/.tauri/myapp.key` 文件
3. 使用 `--signer ~/.tauri/myapp.key` 参数进行签名构建

工作流会根据 `TAURI_SIGNING_PRIVATE_KEY` Secret 是否存在自动选择：
- **如果存在**: 使用签名构建（推荐用于生产环境）
- **如果不存在**: 使用普通构建（仅用于测试）

## 注意事项

⚠️ **安全提示**:
- 永远不要将私钥提交到代码仓库
- 私钥文件 `~/.tauri/myapp.key` 应该添加到 `.gitignore`
- 只有需要发布构建的团队成员才应该访问 GitHub Secrets
- 如果私钥泄露，立即重新生成新的密钥对并更新配置

## 故障排除

### 如果构建失败并提示找不到签名密钥

1. 检查 GitHub Secrets 是否正确配置
2. 确认私钥内容完整（包括所有换行符）
3. 检查工作流日志中的 "Setup signing key" 步骤

### 如果提示密码错误

1. 确认 `TAURI_SIGNING_PRIVATE_KEY_PASSWORD` Secret 的值正确
2. 如果生成密钥时没有设置密码，可以删除此 Secret

### 如果不想使用签名（仅用于测试）

如果 `TAURI_SIGNING_PRIVATE_KEY` Secret 不存在，工作流会自动跳过签名步骤，构建仍然会成功，但生成的更新包将无法通过签名验证。

