use std::{fs::File, io::Write};

use colored::Colorize;

use crate::{arguments::{ExportCommand, Format}, tasks::Task, utility::{conjucture::deserialize, finddir::find_todo_dir}};


pub fn export(cmd: &ExportCommand) -> Result<(), anyhow::Error> {
    let data_path = find_todo_dir()
        .ok_or_else(|| anyhow::anyhow!("No todo found. Run `todo init` to initialize"))?
        .join("todos.bin");
    let task_list = deserialize(&data_path)?;
    match cmd.format {
        Format::CSV => {
            let outpath = if cmd.output.contains(".csv") {
                cmd.output.clone()
            } else {
                cmd.output.clone() + ".csv"
            };
            export_csv(&task_list.tasks, outpath)
        },
        Format::JSON => {
            let outpath = if cmd.output.contains(".json") {
                cmd.output.clone()
            } else {
                cmd.output.clone() + ".json"
            };
            export_json(&task_list.tasks, outpath)
        }
    }
}

fn export_csv(data: &Vec<Task>, name: String) -> Result<(), anyhow::Error> {
    let file = File::create(&name)?;
    let mut wtr = csv::Writer::from_writer(file);
    for unit in data {
        wtr.serialize(unit)?;
    }
    wtr.flush()?;
    println!("CSV file exported at {}", name.green());
    Ok(())
}

fn export_json(data: &Vec<Task>, name: String) -> Result<(), anyhow::Error> {
    let ser_de = serde_json::to_string_pretty(data)?;
    let mut file = File::create(&name)?;
    file.write_all(ser_de.as_bytes())?;
    println!("JSON file exported at {}", name.green());
    Ok(())
}