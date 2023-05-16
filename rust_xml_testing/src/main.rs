use quickxml_to_serde;
use quickxml_to_serde::{xml_string_to_json, Config};

use std::fs;
use std::io::Write;
use std::fs::OpenOptions;
use std::error::Error;
use std::fs::ReadDir;
use std::time::Instant;


fn main() {

    //println!("current working directory: {}", std::env::current_dir().unwrap().display());

    let path_mono = "../xml_datasets/xml_mono";
    let path_micro = "../xml_datasets/xml_micro";
    
    let mono_paths = get_xml_paths(path_mono);
    let micro_paths = get_xml_paths(path_micro);

    // convert mono
    let now = Instant::now();
    let save_path = "../results/rust_results/json_mono/";

    convert_xml(mono_paths, &save_path);

    let total_time = now.elapsed();

    let mono_results_str = "(rust) time for mono (s): ".to_owned() + &total_time.as_secs().to_string() + &"\n" ;
    let mut file = OpenOptions::new()
                    .append(true)
                    .open("../results/benchmark.txt")
                    .unwrap();
    file.write(mono_results_str.as_bytes()).expect("mono benchmark write failed");


    // convert micro
    let now = Instant::now();
    let save_path = "../results/rust_results/json_micro/";

    convert_xml(micro_paths, &save_path);

    let total_time = now.elapsed();
    let micro_results_str = "(rust) time for micro (s): ".to_owned() + &total_time.as_secs().to_string() + &"\n";

    file.write(micro_results_str.as_bytes()).expect("micro benchmark write failed");
}




fn get_xml_paths(source_path: &str) -> ReadDir {
    let paths = fs::read_dir(source_path).unwrap();
    paths
}


fn xml_read(filepath: &String) -> Result<String, Box<dyn Error>> {
    let xml_read = fs::read_to_string(filepath)?;

    Ok(xml_read)
}


fn convert_xml(paths: ReadDir, save_path: &str) {

    // read the xml to string
    // convert xml to json object
    // save the results

    for path in paths {

        
        let path_str = path.expect("error with filepath string").path().to_str().unwrap().to_owned();

        // windows returns a "\"" instead of "/"
        let xml_file_name = &path_str.split(r#"\"#).last().unwrap();
        let json_file_name = [&xml_file_name.split(".").next().unwrap(), ".json"].join("");

        let xml_text = xml_read(&path_str).expect("File Read Issue").to_string();

        let conf = Config::new_with_defaults();
        let json = xml_string_to_json(xml_text.to_owned(), &conf).unwrap().to_string();

        let full_save_path = save_path.to_owned() + &json_file_name;

        fs::write(&full_save_path, &json).expect("unable to write file");

    }
}
