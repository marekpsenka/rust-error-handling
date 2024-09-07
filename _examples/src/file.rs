
#[cfg(test)]
mod tests {
    #[test]
    fn open_nonexistent_file() {
        match std::fs::File::open("non_existent") {
            Ok(file) => drop(file),
            Err(err) => println!("open() failed: {}", err),
        }
    }
}