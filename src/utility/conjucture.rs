use std::{fs::File, path::PathBuf};

use anyhow::Context;

use crate::tasks::TaskList;

pub fn serialize(file: &mut File, data: &TaskList) -> Result<(), anyhow::Error> {
    bincode::encode_into_std_write(data, file, bincode::config::standard())
        .context("Failed to encode todo data")?;
    Ok(())
}

pub fn deserialize(path: &PathBuf) -> Result<TaskList, anyhow::Error> {
    let mut file = File::open(path).context("failed to read data")?;
    let task_list: TaskList = bincode::decode_from_std_read(&mut file, bincode::config::standard())
        .context("failed to decode todo data")?;
    return Ok(task_list);
}