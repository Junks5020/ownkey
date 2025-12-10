## ownkey 项目任务清单（阶段 1 + 阶段 2 前期）

### 【Task 1】完善 CLI 命令体验

* 检查所有 CLI 子命令是否提供合理的 `--help`
* 完善帮助文本和使用示例
* 优化错误提示，避免 panic 或树状错误
* 删除命令增加确认提示

### 【Task 2】增强 CLI 安全性

* 支持隐藏密码输入（不显示在 CLI 上）
* 确保 Vault 保存为加密格式
* 确保 Vault 文件权限为 600
* 警告用户密码给出应用将有安全隐患

### 【Task 3】增强 CLI 错误处理和稳定性

* 替换 unwrap/expect 为合理的 Result 处理
* 确保出现所有异常情况不会引起系统崩溃
* 确保 CLI 退出时 exit code 合理

### 【Task 4】扩展测试覆盖率

* 为重点功能写单元测试（vault 加密/解密）
* 为 CLI 子命令写集成测试
* 处理边界情况（无数据，错误输入，破损文件等）

### 【Task 5】完善项目文档

* 补全 README: 包括安装方法、使用示例
* 新增 CONTRIBUTING.md: 提交步骤
* 新增 SECURITY.md: 加密方案、密钥模型说明
* 新增 ROADMAP.md: 队列后续计划

---

### 【Task 6】新增 sync_provider 配置支持

* 创建 `Config` 结构体，启动时读取 ~/.ownkey/config.toml
* 支持属性 `sync_provider: "local_only" | "file" | "http"`
* 如配置文件不存在，创建模板配置

### 【Task 7】定义 SyncBackend trait

* 描述定义 trait：login/logout/is_logged_in/pull_vault/push_vault
* 设计 SyncError 错误类型：网络错误/文件错误/未登录等
* 配合 Config 实现各同步后端切换

### 【Task 8】实现 NoopSyncBackend

* 完成 SyncBackend trait 的空实现，所有操作返回 Ok()
* 在 sync_provider = "local_only" 时启用

### 【Task 9】添加 CLI 同步命令

* 添加 CLI 命令 `ownkey login`，`ownkey logout`，`ownkey sync`
* 默认输出 Not implemented yet
* 调用选择的 SyncBackend 实现

### 【Task 10】实现 FileSyncBackend

* 通过配置 sync_provider = "file" 时启用
* login: 检测路径存在并创建 remote_vault.json
* logout: 空操作
* push_vault: 把本地 vault.json 备份到 remote
* pull_vault: 从 remote 覆盖本地 vault

### 【Task 11】为 sync 功能写全套测试

* 重点测试 FileSyncBackend login/push/pull 行为
* CLI sync 测试：添加/删除后同步结果如期
* 设置不同 sync_provider 配置验证切换行为
* 验证错误场景：路径不可写/文件不存在/未登录
