/* tslint:disable */
/* eslint-disable */

export class PathfindingWasm {
    free(): void;
    [Symbol.dispose](): void;
    create_zone(zone_id: string, positions: Float32Array, indices: Uint32Array, tolerance: number): void;
    create_zone_handle(positions: Float32Array, indices: Uint32Array, tolerance: number): number;
    find_path(zone_id: string, group_id: number, start_x: number, start_y: number, start_z: number, target_x: number, target_y: number, target_z: number, output: Float32Array): number;
    get_closest_node_id(zone_id: string, x: number, y: number, z: number, check_polygon: boolean): number | undefined;
    get_group(zone_id: string, x: number, y: number, z: number, check_polygon: boolean): number | undefined;
    get_group_by_handle(zone_handle: number, x: number, y: number, z: number, check_polygon: boolean): number | undefined;
    group_count(zone_id: string): number | undefined;
    group_count_by_handle(zone_handle: number): number | undefined;
    group_node_centers(zone_id: string, group_id: number): Float64Array | undefined;
    group_node_count(zone_id: string, group_id: number): number | undefined;
    group_node_ids(zone_id: string, group_id: number): Uint32Array | undefined;
    constructor();
    node_center(zone_id: string, group_id: number, node_id: number): Float64Array | undefined;
}
