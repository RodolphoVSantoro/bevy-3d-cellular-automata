pub fn should_decay(neighbors: u32) -> bool {
    return neighbors > 3 || neighbors < 2;
}

pub fn should_spawn(neighbors: u32) -> bool {
    return neighbors == 2;
}
