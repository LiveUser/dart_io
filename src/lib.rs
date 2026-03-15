use std::fs;
use std::path::{Path, PathBuf};

//Directory----------------------
pub struct Directory{
    full_path:String,
}
impl Directory {
    //Directory exists
    pub fn exists(&self) -> bool {
        let dir_exists:bool = Path::new(&self.full_path).is_dir();
        if dir_exists{
            true
        }else {
            false
        }
    }
    //create dir sync
    pub fn create_sync(&self) -> bool{
        match fs::create_dir_all(&self.full_path) {
            Err(_)=>{
                false
            },
            Ok(_)=>{
                true
            },
        }
    }
    //delete dir sync
    pub fn delete_sync(&self) -> bool{
        match fs::remove_dir_all(&self.full_path) {
            Err(_)=>{
                false
            },
            Ok(_)=>{
                true
            },
        }
    }
    //List directory contents
    pub fn list_contents(&self) -> Vec<FileSystemEntity> {
        let mut folder_contents:Vec<FileSystemEntity> = Vec::new();
        match fs::read_dir(&self.full_path) {
            Err(_)=>{
                folder_contents
            },
            Ok(items)=>{
                for item in items{
                    if let Ok(entity) = item {
                        let path:PathBuf = entity.path();
                        if path.is_dir(){
                            folder_contents.push(FileSystemEntity::Directory(
                                Directory{
                                    full_path: path.to_string_lossy().to_string(),
                                }
                            ));
                        }else {
                            folder_contents.push(FileSystemEntity::File(
                                File{
                                    full_path: path.to_string_lossy().to_string(),
                                }
                            ));
                        }
                    }
                }
                folder_contents
            },
        }
    }
}

//File----------------------------
pub struct File{
    full_path:String,
}
impl File{
    //File exists
    pub fn exists(&self) -> bool {
        let file_exists:bool = Path::new(&self.full_path).is_file();
        if file_exists{
            true
        }else {
            false
        }
    }
    //create file sync
    pub fn create_sync(&self) -> bool {
        //Create parent folders
        match &self.full_path.rfind("/") {
            None => {
                return false
            },
            Some(index)=>{
                let dir_path:String = self.full_path[0..*index].to_string();
                let save_directory:Directory = Directory { 
                    full_path: dir_path,
                };
                let _ = save_directory.create_sync();
                //Create file
                match std::fs::File::create(&self.full_path) {
                    Err(_)=>{
                        false
                    },
                    Ok(_)=>{
                        true
                    }
                }
            },
        }
    }
    //delete file sync
    pub fn delete_sync(&self) -> bool {
        match std::fs::remove_file(&self.full_path) {
            Err(_)=>{
                false
            },
            Ok(_)=>{
                true
            }
        }
    }
    //Write as bytes
    pub fn write_as_bytes(&self, bytes:Vec<u8>) -> bool{
        match fs::write(&self.full_path, bytes){
            Err(_)=>{
                false
            },
            Ok(_)=>{
                true
            },
        }
    }
    //write as string
    pub fn write_as_string(&self, contents:String) -> bool{
        match fs::write(&self.full_path, &contents){
            Err(_)=>{
                return false
            },
            Ok(_)=>{
                return true
            },
        }
    }
    //Read as bytes
    pub fn read_as_bytes(&self) -> Vec<u8>{
        match fs::read(&self.full_path) {
            Err(_)=>{
                [].to_vec()
            },
            Ok(bytes)=>{
                bytes
            },
        }
    }
    //read as string
    pub fn read_as_string(&self) -> String{
        match fs::read_to_string(&self.full_path){
            Err(_)=>{
                return "".to_string()
            },
            Ok(content)=>{
                return content
            },
        }
    }
}

pub enum FileSystemEntity{
    File(File),
    Directory(Directory),
}



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn directory_checks() {
        let test_directory:Directory = Directory { 
            full_path: String::from("./test"),
        };
        //Exists
        println!("{} exists = {}",test_directory.full_path, test_directory.exists());
        //Create folder
        test_directory.create_sync();
        //Check existance
        println!("{} exists = {}",test_directory.full_path, test_directory.exists());
        //Delete folder
        test_directory.delete_sync();
        //Check existance again
        println!("{} exists = {}",test_directory.full_path, test_directory.exists());
    }
    #[test]
    fn list_folder_contents(){
        let file:File = File { 
            full_path:  String::from("./test/file.txt"),
        };
        let file2:File = File { 
            full_path:  String::from("./test/file2.txt"),
        };
        let folder1:Directory = Directory { 
            full_path: String::from("./test/folder"),
        };
        file.create_sync();
        file2.create_sync();
        folder1.create_sync();
        let test_directory:Directory = Directory { 
            full_path: String::from("./test"),
        };
        let folder_contents:Vec<FileSystemEntity> = test_directory.list_contents();
        for file_system_entity in folder_contents{
            match file_system_entity {
                FileSystemEntity::File(file)=>{
                    println!("File: {}", file.full_path);
                },
                FileSystemEntity::Directory(directory)=>{
                    println!("Directory: {}", directory.full_path);
                },
            }
        }
    }
    #[test]
    fn file_checks() {
        //Create file
        let file:File = File { 
            full_path:  String::from("./test/file.txt"),
        };
        file.create_sync();
        //Write as string
        file.write_as_string("Hello World".to_string());
        //File exists
        println!("file exists = {}",file.exists());
        //Read as string
        let text:String = file.read_as_string();
        println!("{text}");
        //Delete file
        file.delete_sync();
    }
    #[test]
    fn file_checks_bytes(){
        //Create file
        let file:File = File { 
            full_path:  String::from("./test/file.txt"),
        };
        file.create_sync();
        //Write as string
        file.write_as_bytes([1,2,3,4,5].to_vec());
        //File exists
        println!("file exists = {}",file.exists());
        //Read as string
        let bytes:Vec<u8> = file.read_as_bytes();
        println!("{:?}", bytes);
        //Delete file
        file.delete_sync();
    }
}
