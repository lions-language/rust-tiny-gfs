## 组成部分
1. chunkserver
- 主要负责存储实际的数据
- 每一块数据记为一个 chunk (默认大小为 64M), 并且每一个chunk存在一个唯一ID

2. 目录master
- 负责记录 chunk 的位置
- 目录数据存储在内存, 所以存在丢失的风险
    - 使用 checkpoints 和 操作日志 的方式, 做到单机重启后的快速恢复

3. 备份-目录master
- 防止 单机 目录 master 磁盘损坏, 需要进行目录数据的备份

4. 监控程序
- 目录master 和 备份-目录master

5. 影子-目录master
- 异步复制 目录master 的 服务, 主要为了防止 主备(目录master 和 备份-目录master)切换时, 无法对外提供服务的问题


## chunkserver
1. 小文件存储方式
- 小文件存储在一个 chunk 中, master 记录下 `chunk-id`和偏移

2. 小文件查找方式

3. 大文件的存储方式

4. 大文件的查找方式


## 目录master
### 需要记录的数据内容
1. 命名空间+文件名 和 `chunk-id` 的关系
- 因为要存储小文件, 所以一个 `chunk-id` 中会存在多个偏移, 用于表示不同的 小文件
2. `chunk-id`和所有 chunkserver 副本的关系

