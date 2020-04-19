//
// Copyright (C) 2020 BrdgAI, Inc.
// This file is subject to the terms and conditions defined in
// the file 'LICENSE', included as part of this source code package.
//

use structopt::StructOpt;

fn do_backup(
    source_db: &str,
    dest_db: &str,
    page_size: i32,
    sleeptime: u64,
    verbose: bool,
    dry_run: bool,
) -> Result<(), rusqlite::Error> {
    if verbose {
        println!("Backing up {} to {}", source_db, dest_db);
    }
    let src = rusqlite::Connection::open_with_flags(
        source_db,
        rusqlite::OpenFlags::SQLITE_OPEN_READ_ONLY,
    )?;
    let mut dst = rusqlite::Connection::open(dest_db)?;
    if dry_run {
        println!("Dry run exiting.  Opened DB handles successfully.");
        return Ok(());
    }
    let backup = rusqlite::backup::Backup::new(&src, &mut dst)?;
    backup.run_to_completion(page_size, std::time::Duration::from_millis(sleeptime), None)?;
    if verbose {
        println!("Backup complete");
    }
    Ok(())
}

#[derive(Debug, StructOpt)]
#[structopt(name = "sqlite_backup", about = "Copy a sqlite db to a backup file")]
struct Opt {
    #[structopt(short, long)]
    verbose: bool,

    #[structopt(short = "n", long, help = "Do not actually copy data")]
    dry_run: bool,

    #[structopt(short, long, help = "Set page copy size", default_value = "4096")]
    page_size: i32,

    #[structopt(
        short,
        long,
        help = "Sleep time between groups of pages copied, in ms",
        default_value = "10"
    )]
    sleeptime: u64,

    source_db: String,
    dest_db: String,
}

fn main() {
    let opt = Opt::from_args();

    if let Err(e) = do_backup(
        &opt.source_db,
        &opt.dest_db,
        opt.page_size,
        opt.sleeptime,
        opt.verbose,
        opt.dry_run,
    ) {
        eprintln!("Error backing up database: {}", e);
        std::process::exit(1);
    }
}
