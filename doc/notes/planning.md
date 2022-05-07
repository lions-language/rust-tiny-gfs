## 实现路线
1. 一个 目录master + 多个 chunkserver
- 目录 master 采用内存存储
    - 不提供任何可靠性保证
- chunkserver 只负责顺序写入 和 提供数据获取

