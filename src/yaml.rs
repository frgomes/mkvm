use std::fs::File;
use std::io::prelude::*;
use yaml_rust::yaml;


pub fn files(files: &Vec<&str>) {
    for file in files {
        println!("define vm {}", file);
        let mut f = File::open(file).unwrap();
        let mut s = String::new();
        f.read_to_string(&mut s).unwrap();
        let docs = yaml::YamlLoader::load_from_str(&s).unwrap();
        for doc in &docs {
            println!("---");
            dump(doc, 0);
        }
    }
}




fn dump(doc: &yaml::Yaml, indent: usize) {
    match *doc {
        yaml::Yaml::Array(ref v) => {
            for x in v {
                dump(x, indent + 1);
            }
        }
        yaml::Yaml::Hash(ref h) => {
            for (k, v) in h {
                blanks(indent);
                println!("HHHH {:?}:", k);
                dump(v, indent + 1);
            }
        }
        _ => {
            blanks(indent);
            println!("???? {:?}", doc);
        }
    }
}

fn blanks(size: usize) {
    for _ in 0..size {
        print!("    ");
    }
}


#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        println!("it works")
    }
}
