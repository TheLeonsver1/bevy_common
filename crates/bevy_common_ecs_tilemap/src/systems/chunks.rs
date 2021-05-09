use crate::data::{chunks::components::*, tiles::components::*};
use bevy::{
    prelude::*,
    utils::{HashMap, HashSet},
};
use bevy_ecs_tilemap::prelude::*;
pub fn update_chunk_path_node_grid(
    changed_tiles_query: Query<&Tile, Or<(Added<PathNodes>, Changed<PathNodes>)>>,
    tiles_with_path_nodes: Query<(&Tile, &UVec2, &PathNodes, &GlobalTransform)>,
    mut chunk_query: Query<(&Chunk, &mut ChunkPathingNodes)>,
) {
    //Get which chunks need to get updated and which cells of them in the spatial grid
    let mut chunks_that_need_update: HashSet<Entity> = HashSet::default();
    chunks_that_need_update.reserve(chunk_query.iter_mut().len());
    //TODO:optimization potential(maybe, it might be worse): add the cell which the chunk needs to update
    for tile in changed_tiles_query.iter() {
        chunks_that_need_update.insert(tile.chunk);
    }
    //For every chunk entity that needs an update
    for chunk_entity in chunks_that_need_update {
        //If it exists (and it should..)
        if let Ok((chunk, mut chunk_pathing_nodes)) = chunk_query.get_mut(chunk_entity) {
            //Get the tiles the chunk is made of
            let chunk_tiles = chunk.get_tiles();
            //TODO: optimization: This should perhaps be a component to add to ChunkBundle, this is lightweight..
            let chunk_first_tile = UVec2::new(
                chunk.settings.position.x * chunk.settings.size.x,
                chunk.settings.position.y * chunk.settings.size.y,
            );
            //Reset the chunks spatial node map
            for (key, vec) in chunk_pathing_nodes.node_spatial_map.iter_mut() {
                *vec = Vec::<Vec2>::with_capacity(vec.len());
            }
            //For every optional tile
            for tile_entity_option in chunk_tiles.iter() {
                //If it exists
                if let Some(tile_entity) = tile_entity_option {
                    //And it has a PathNodes component
                    if let Ok((tile, position, path_nodes, global_transform)) =
                        tiles_with_path_nodes.get(*tile_entity)
                    {
                        //Calculate its position in the chunk
                        let position_in_chunk = *position - chunk_first_tile;
                        //Calculate its spatial_cell in the chunk
                        let tile_cell = position_in_chunk / 10;
                        //Calculate the length of the tile in world space
                        let length = Vec2::new(
                            chunk.settings.tile_size.x * global_transform.scale.x,
                            chunk.settings.tile_size.y * global_transform.scale.y,
                        );
                        //Get the tiles (0,0) position, we need it to know where to put the node in the world
                        let zero_position = Vec2::new(
                            global_transform.translation.x - length.x / 2.0,
                            global_transform.translation.y - length.y / 2.0,
                        );
                        //For every node
                        for node in path_nodes.nodes.iter() {
                            chunk_pathing_nodes
                                .node_spatial_map
                                //Get it's entry so we could modify it
                                .entry(tile_cell)
                                .and_modify(|vec| {
                                    //Push the node into the cell
                                    vec.push(
                                        zero_position
                                            + Vec2::new(node.x * length.x, node.y * length.y),
                                    )
                                });
                        }
                    }
                }
            }
        }
    }
}
