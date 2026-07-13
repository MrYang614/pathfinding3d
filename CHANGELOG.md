# Changelog

All notable changes to this project are documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/).

## [Unreleased]

### Changed

- **Node IDs are now group-local.** Triangle IDs returned by `get_closest_node_id`, `group_node_ids`, and accepted by `node_center` are indices within a single connected group (`0` … `group_node_count - 1`), not global triangle indices across the entire zone. Always pair a node ID with the `groupId` from `get_group` (or the `groupId` argument you pass to `find_path` / metadata APIs).
- **A\* edge cost** now uses Euclidean distance between triangle centroids for both `g` and `h`, fixing inconsistent metrics that could produce suboptimal paths.

### Migration

If you previously stored node IDs from `get_closest_node_id` or `group_node_ids` as global indices:

1. Resolve the agent's group with `get_group(zoneId, x, y, z, checkPolygon)`.
2. Use the returned `groupId` with `group_node_ids`, `node_center`, and `find_path`.
3. Treat node IDs as **per-group** indices only; do not compare or reuse them across different groups.

## [1.0.1] - 2026-03-01

Initial npm release with WASM pathfinding API.

## License

This project is licensed under the [MIT License](LICENSE).
