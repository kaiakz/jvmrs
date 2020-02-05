use std::io::prelude::*;
use std::fs::File;
use std::io;
use std::io::BufReader;
use crate::utils::bytes::Bytes;
use crate::utils::bytes::ByteStream;

pub struct ClassPath {
    classpath : Vec<String>
}

impl ClassPath {
    pub fn new() -> ClassPath {
        let cp = ClassPath{classpath : Vec::new()};
        cp
    }

    pub fn add(&mut self, s:String) {
        for i in s.split(";") {
            if i != "" {
                self.classpath.push(i.to_string());
            }
        }        
    }

    pub fn fetch(&self, class_name:&str) -> Bytes {
        let mut content : Bytes = Vec::new();
        for i in &self.classpath {
            if i.ends_with(".jar") || i.ends_with(".JAR") ||
                i.ends_with(".zip") || i.ends_with(".ZIP") {
                    content = match ClassPath::jar(i, &class_name) {
                        Ok(content) => {
                            // if !content.is_empty() {
                            //     for i in &content {
                            //         print!("{} ", i);
                            //     }                    
                            // }
                            content                     
                        },
                        Err(_e) => continue,
                    };

            } else if i.ends_with("*") {
                // wildcard(i);

            } else {
                content = match ClassPath::dir(i, &class_name) {
                    Ok(content) => {
                        // if !content.is_empty() {
                        //     for i in &content {
                        //         print!("{} ", i);
                        //     }                    
                        // }
                        content                   
                    },
                    Err(_e) => continue,
                };

                // if !content.is_empty() {
                //     for i in &content {
                //         print!("{} ", i);
                //     }                    
                //     break;
                // }
            }
        };
        content
    }

    fn dir(abs_path:&str, class_name:&str)-> Result<Bytes, io::Error> {
        let mut clspath = String::new();
        clspath = clspath + &abs_path + &class_name;
        let mut buf : Bytes = Vec::new();
        // if std::path::Path::new(&clspath).exists() {
        //     // let mut f = File::open(clspath).unwrap();
        //     // f.read_to_end(&mut buf).unwrap();
              
        // }
        File::open(clspath)?.read_to_end(&mut buf)?;
        Ok(buf)
    }

    fn jar(abs_jar_path:&str, class_name:&str) -> Result<Bytes, zip::result::ZipError> {
        let mut buf : Bytes = Vec::new();
        zip::ZipArchive::new(BufReader::new(File::open(&abs_jar_path)?))?.by_name(&class_name)?.read_to_end(&mut buf)?;
        Ok(buf)

        // for i in 0..zip.len() {
        //     let mut file = zip.by_index(i).unwrap();
        //     // Do something with file i
        // }
    }
}