# sqlite_backup
This small utility is a small wrapper around the rust sqlite3 backup API.
Unlike using the `.backup` command from the sqlite shell, this utility
lets you specify the page copy size and the sleep duration between
pages to make it easier to identify a good tradeoff between copy
time and the time the database is locked.

This utility will overwrite the contents of the destination database
file.

## Building
To build this repository, check it out and run:

```
  cargo build --release
```

You will need to have the sqlite libraries and headers installed.  On ubuntu, this is:

```sudo apt install libsqlite3-dev```

## Use
```
sqlite_backup 0.1.0
Copy a sqlite db to a backup file

USAGE:
    sqlite_backup [FLAGS] [OPTIONS] <source-db> <dest-db>

FLAGS:
    -n, --dry-run    Do not actually copy data
    -h, --help       Prints help information
    -V, --version    Prints version information
    -v, --verbose

OPTIONS:
    -p, --page-size <page-size>    Set page copy size [default: 4096]
    -s, --sleeptime <sleeptime>    Sleep time between groups of pages copied, in ms [default: 10]

ARGS:
    <source-db>
    <dest-db>
```
