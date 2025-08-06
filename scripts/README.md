# Toolcraft Release Scripts

这个目录包含用于管理和发布 toolcraft 各个 crate 的脚本。

## release.sh - 发布脚本

这个脚本帮助管理工作空间中每个 crate 的独立版本控制。

### 使用方法

```bash
./scripts/release.sh <package-name> <version> [--dry-run]
```

### 参数说明

- `<package-name>`: 要发布的包名（不含 crates/ 前缀）
- `<version>`: 新版本号（例如：0.2.1）
- `[--dry-run]`: 可选参数，执行测试运行而不实际发布

### 可用的包

- `toolcraft-jwt` - JWT 认证库
- `toolcraft-axum-kit` - Axum web 服务工具包
- `toolcraft-request` - HTTP 客户端封装
- `toolcraft-config` - 配置管理库

### 使用示例

#### 1. 测试发布（推荐先执行）
```bash
# 测试发布 toolcraft-jwt 0.2.1
./scripts/release.sh toolcraft-jwt 0.2.1 --dry-run
```

#### 2. 实际发布
```bash
# 发布 toolcraft-jwt 0.2.1
./scripts/release.sh toolcraft-jwt 0.2.1
```

### 发布顺序

由于包之间存在依赖关系，请按以下顺序发布：

1. **toolcraft-jwt** - 基础包，无内部依赖
2. **toolcraft-axum-kit** - 依赖 toolcraft-jwt
3. **toolcraft-request** - 独立包

### 完整的发布流程

1. **准备工作**
   ```bash
   # 确保登录 crates.io
   cargo login
   
   # 确保所有更改已提交
   git status
   ```

2. **发布 toolcraft-jwt**
   ```bash
   # 先测试
   ./scripts/release.sh toolcraft-jwt 0.2.1 --dry-run
   
   # 确认无误后实际发布
   ./scripts/release.sh toolcraft-jwt 0.2.1
   ```

3. **更新依赖版本**（如果需要）
   ```bash
   # 如果其他包需要使用新版本的 toolcraft-jwt
   # 手动编辑 crates/toolcraft-axum-kit/Cargo.toml
   # 更新: toolcraft-jwt = { version = "0.2.1", path = "../toolcraft-jwt" }
   ```

4. **发布 toolcraft-axum-kit**
   ```bash
   ./scripts/release.sh toolcraft-axum-kit 0.2.3 --dry-run
   ./scripts/release.sh toolcraft-axum-kit 0.2.3
   ```

5. **发布 toolcraft-request**
   ```bash
   ./scripts/release.sh toolcraft-request 0.2.3 --dry-run
   ./scripts/release.sh toolcraft-request 0.2.3
   ```

6. **推送更改**
   ```bash
   # 推送代码
   git push
   
   # 推送标签
   git push --tags
   ```

### 脚本功能

- ✅ 自动更新版本号
- ✅ 检查包是否存在
- ✅ 显示当前版本和新版本
- ✅ 依赖检查（针对 toolcraft-axum-kit）
- ✅ 确认提示，防止误操作
- ✅ 彩色输出，便于阅读
- ✅ 失败时自动回滚版本更改

### 注意事项

1. **依赖版本管理**：脚本不会自动更新依赖版本，需要手动更新
2. **发布顺序**：确保先发布被依赖的包
3. **标签格式**：每个包使用不同的标签前缀
   - toolcraft-jwt: `jwt-v{version}`
   - toolcraft-axum-kit: `axum-kit-v{version}`
   - toolcraft-request: `request-v{version}`

### 故障排除

如果发布失败：

1. 检查是否已登录 crates.io：`cargo login`
2. 检查工作区是否干净：`git status`
3. 检查依赖版本是否已发布
4. 查看详细错误信息并根据提示解决