use std::fs::File;

pub fn errors() {
    // The return type of File::open is a Result<T, E>.
    // In the case where File::open succeeds, the value in the variable greeting_file_result will be an instance of Ok that contains a file handle. In the case where it fails, the value in greeting_file_result will be an instance of Err that contains more information about the kind of error that happened.
    let greeting_file_result = File::open("hello.txt");
    
    let greeting_file = match greeting_file_result {
        Ok(file) => file, //When the result is Ok, this code will return the inner file
        Err(error) => panic!("Problem opening the file: {:?}", error), // error can be any variable
    };
}