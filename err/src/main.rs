use std::fs;

fn main() {
    use std::fs::File;
    use std::io::{self, Read};

    fn read_username_from_file() -> Result<String, io::Error> {
        let username_file_result = File::open("hello.txt");

        let mut username_file = match username_file_result {
            Ok(file) => file,
            Err(e) => return Err(e),
        };

        let mut username = String::new();

        match username_file.read_to_string(&mut username) {
            Ok(_) => Ok(username),
            Err(e) => Err(e),
        }
    }
    // let name = match read_username_from_file() {
    //     Ok(v) => v,
    //     Err(e) => panic!("{}", e),
    // };
    // println!("{}", name);
    
    fn read_username_from_file2() -> Result<String, io::Error> {
        let mut username_file = File::open("hello.txt").unwrap();
        let mut username = String::new();
        username_file.read_to_string(&mut username)?;
        Ok(username)
    }
    println!("{}", read_username_from_file2().unwrap());

    fn read_username_from_file3() -> Result<String, io::Error> {
        let mut username = String::new();
        File::open("hello.txt")?.read_to_string(&mut username)?;
        Ok(username)
    }
    println!("read_username_from_file3() = {}", read_username_from_file3().unwrap());

    fn read_username_from_file4() -> Result<String, io::Error> {
        // fs::read_to_string("hello.txt")?;
        fs::read_to_string("hello.txt")

    }
    println!("read_username_from_file4() = {}", read_username_from_file4().unwrap());

    fn last_char_of_first_line(text: &str) -> Option<char> {
        // println!("{:?}", text.lines().next()?);
        text.lines().next()?.chars().last()
    }

    assert_eq!(
        last_char_of_first_line("Hello, world\nHow are you today?"),
        Some('d')
    );

    assert_eq!(last_char_of_first_line(""), None);
    assert_eq!(last_char_of_first_line("\nhi"), None);
}

// note:
// unwrap and ? usage