pub mod io {
    use std::{fs::File, io::Read};

    fn read(path: &String) -> Result<String, Box<dyn std::error::Error>> {
        let mut file = File::open(path)?;
        let mut content = String::new();

        file.read_to_string(&mut content)?;

        Ok(content)
    }

    pub fn read_test(day: &str) -> Result<String, Box<dyn std::error::Error>> {
        let path = format!("src/{day}/test.txt");
        read(&path)
    }

    pub fn read_input(day: &str) -> Result<String, Box<dyn std::error::Error>> {
        let path = format!("src/{day}/input.txt");
        read(&path)
    }
}