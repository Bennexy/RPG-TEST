// weigthed tile choices
/* 
let get_evenly_distributed_tile_choices = |image_handles: &Vec<Handle<Image>>| {

    let weigth_per_tile = 1 as f64 / image_handles.len() as f64;
    let tile_choices_weighted_dyn: Vec<(TileTextureIndex, f64)> = image_handles
        .iter()
        .enumerate()
        .map(|(index, _val)| {(TileTextureIndex(index as u32), weigth_per_tile)})
        .collect();
    return tile_choices_weighted_dyn;
};

let weighted_tile_choices = get_evenly_distributed_tile_choices(&image_handles);
*/