use anyhow::{Result, Context};
use std::{
    fs::OpenOptions,
    io::{
        Read,
        Write,
        Seek,
        SeekFrom,
    },
};

pub fn chop(path: &str, part_count: &u64, slice_size: &u64) -> Result<()> {
    // Metadata like perms is likely gonna have to be OS specific, e.g. std::os::unix::fs::PermissionsExt

    println!("{}, {}, {}", path, part_count, slice_size);
    //exit(0);

    let mut file = OpenOptions::new()
        .read(true)
        .write(true)
        .open(path)?;
    let mut file_size = file.metadata()?.len();
    let mut buffer = vec![];
    let mut part_count = *part_count;

    while part_count > 0 {
        // Can't slice from end as it's not guaranteed we can make a u64 an i64
        file.seek(SeekFrom::Start(file_size - slice_size)).unwrap_or_else(|_| { eprintln!("Failed to seek"); 0 });
        file.read_to_end(&mut buffer)
            .context("Failed to read part of file")?;
        let mut write_file = OpenOptions::new()
            .create_new(true) // Fail if there's already a file there, we don't want to mess things up
            .write(true)
            .open(format!("{}.chop{}", path, part_count))
            .context("Failed to create a chop part file (does it already exist?)")?;
        write_file.write_all(&buffer)?;

        file_size -= slice_size;
        file.set_len(file_size).context("Failed to truncate file")?;
        part_count -= 1;
    }

    Ok(())
}
