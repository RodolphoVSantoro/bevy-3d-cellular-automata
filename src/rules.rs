pub fn should_decay(neighbors: u32) -> bool {
    return neighbors < 2 || neighbors > 5;
}

pub fn should_spawn(neighbors: u32) -> bool {
    return neighbors == 1;
}
