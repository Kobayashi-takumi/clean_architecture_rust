use crate::interface_adapter::gateway::task::TaskEntity;
use crate::shared::error::{Error, Result};
use csv::{ReaderBuilder, Writer};
use std::fs::{File, OpenOptions};
use std::str::FromStr;
use uuid::Uuid;

pub mod query;
pub mod repository;

const TASK_FILE_PATH: &str = "task.csv";

fn open_file(path: &str) -> Result<File> {
    OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .open(path)
        .map_err(|e| {
            log::error!("{e}");
            Error::Unknown
        })
}

fn output_csv(path: &str, header: Vec<String>, data: Vec<Vec<String>>) -> Result<()> {
    let file = File::create(path).map_err(|e| {
        log::error!("{e}");
        Error::Unknown
    })?;
    let mut wtr = Writer::from_writer(file);
    wtr.write_record(header).map_err(|e| {
        log::error!("{e}");
        Error::Unknown
    })?;
    for row in data {
        wtr.write_record(row).map_err(|e| {
            log::error!("{e}");
            Error::Unknown
        })?;
    }
    wtr.flush().map_err(|e| {
        log::error!("{e}");
        Error::Unknown
    })
}

fn tasks_from_csv(file: File) -> Result<Vec<TaskEntity>> {
    let mut res: Vec<TaskEntity> = vec![];
    let mut rdr = ReaderBuilder::new().has_headers(true).from_reader(file);
    for r in rdr.records() {
        let record = r.map_err(|e| {
            log::error!("{e}");
            Error::Unknown
        })?;
        let id = &record[0];
        let id = Uuid::from_str(id).map_err(|e| {
            log::error!("{e}");
            Error::InvalidFormat("ID".to_string())
        })?;
        let title: String = record[1].to_string();
        let description: String = record[2].to_string();
        let is_completed = match &record[3] {
            "true" | "True" | "TRUE" => true,
            "false" | "False" | "FALSE" => false,
            _ => return Err(Error::InvalidFormat("is_completed ".to_string())),
        };
        let created_at = record[4].parse::<i64>().map_err(|e| {
            log::error!("{e}");
            Error::InvalidFormat("created_at".to_string())
        })?;
        let updated_at = record[4].parse::<i64>().map_err(|e| {
            log::error!("{e}");
            Error::InvalidFormat("updated_at ".to_string())
        })?;
        res.push(TaskEntity {
            id,
            title,
            description,
            is_completed,
            created_at,
            updated_at,
        })
    }
    Ok(res)
}
