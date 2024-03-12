use std::sync::atomic::AtomicU32;

use chrono::Local;
use tokio::fs::File;
use tokio::io::{AsyncWriteExt, Error};

pub static FILE_COUNTER: AtomicU32 = AtomicU32::new(0);
   
   
 pub  fn generate_file_name(counter: u32) -> String {
       // Obtener la fecha actual
       let current_date = Local::now();
       let formatted_date = current_date.format("%d%m%Y").to_string();
   
       // Concatena la fecha y el contador para formar el nombre del archivo
       let file_name = format!("{}_{}.DAT", formatted_date, counter);
   
       file_name
   }

pub async fn save_data(file: &mut File, data: &str) -> Result<(), Error> {
    file.write_all(data.as_bytes()).await?;
    Ok(())
}