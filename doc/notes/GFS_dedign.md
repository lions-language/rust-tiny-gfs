## 组成部分
1. chunkserver
- 主要负责存储实际的数据
- 每一块数据记为一个 chunk (默认大小为 64M), 并且每一个chunk存在一个唯一ID

2. 目录 master
- 负责记录 chunk 的位置
