# Dart IO
dart_io is a dart inspired high level abstraction to interact with io operations. Hecho en Puerto Rico por Radamés Jomuel Valentín Reyes.

## Directory
### Directory Struct
~~~rs
let test_directory:Directory = Directory { 
    full_path: String::from("./test"),
};
~~~
#### Directory exists
~~~rs
test_directory.exists()
~~~
#### Create Directory
~~~rs
test_directory.create_sync();
~~~
#### Delete Directory
~~~rs
test_directory.delete_sync();
~~~
#### List Directory Contents
~~~rs
let folder_contents:Vec<FileSystemEntity> = test_directory.list_contents();
~~~
## File
### File Struct
~~~rs
let file:File = File { 
    full_path:  String::from("./test/file.txt"),
};
~~~
#### File exists
~~~rs
file.create_sync();
~~~
#### Create File
~~~rs
file.create_sync();
~~~
#### Delete Directory
~~~rs
file.delete_sync();
~~~
#### Read File as String
~~~rs
let text:String = file.read_as_string();
~~~
#### Read File as Bytes
~~~rs
let bytes:Vec<u8> = file.read_as_bytes();
~~~