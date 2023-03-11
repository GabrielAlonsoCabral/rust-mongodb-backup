# rust-mongodb-backup
  Rust application binary to backup collections in MongoDB.  

  Developed by: <a href="https://www.github.com/gabrielAlonsoCabral">@GabrielAlonsoCabral</a>  
 <br/>

## Installation

```
# clone this repository
$ git clone https://github.com/GabrielAlonsoCabral/rust-mongodb-backup.git
$ cd rust-mongodb-backup
```

<br/>


## Usage

```
# Insert Documents in Generical Book collection
$ cargo run --bin insert-many

# Build all binaries
$ cargo build --release

# Backup all collections 
$ cargo run --bin backup
```


## Build

```
$ cargo build --release
```

## Running binary application

```
$ ./target/release/backup 
```
