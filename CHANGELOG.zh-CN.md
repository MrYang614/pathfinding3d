# 更新日志

本项目的所有重要变更均记录于此。

格式参考 [Keep a Changelog](https://keepachangelog.com/zh-CN/1.1.0/)。

## [未发布]

### 变更

- **节点 ID 改为组内局部索引。** `get_closest_node_id`、`group_node_ids` 返回的三角形 ID，以及 `node_center` 接受的 `nodeId`，均为单个连通分组内的下标（`0` … `group_node_count - 1`），而不再是整个 Zone 的全局三角形序号。请始终将节点 ID 与 `get_group` 返回的 `groupId`（或传给 `find_path` / 元数据 API 的 `groupId`）配合使用。
- **A\* 边代价** 的 `g` 与 `h` 均改为三角形质心之间的欧氏距离，修复此前 `g` 用距离平方、`h` 用距离导致路径可能非最优的问题。

### 迁移说明

若你此前将 `get_closest_node_id` 或 `group_node_ids` 的返回值当作全局索引使用：

1. 用 `get_group(zoneId, x, y, z, checkPolygon)` 解析代理所在分组。
2. 将得到的 `groupId` 与 `group_node_ids`、`node_center`、`find_path` 一起使用。
3. 节点 ID 仅在**同一分组内**有意义，不可跨组比较或复用。

## [1.0.1] - 2026-03-01

首个 npm 发布版本，提供 WASM 寻路 API。

## 许可

本项目采用 [MIT 许可证](LICENSE)。
