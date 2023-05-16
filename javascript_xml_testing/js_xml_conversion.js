import { xml2json } from 'xml-js';
//https://www.npmjs.com/package/xml-js
import { readFileSync, readdirSync, writeFileSync, appendFileSync } from 'fs';
import { join } from 'path';


function convert_xml(load_file_path, save_file_path) {
    // function to 
    var options = {compact: true, ignoreComment: true, spaces: 4};

    var files = get_file_names(load_file_path);

    //console.log(files)

    for (const file of files) {

        var file_xml = readFileSync( `${load_file_path}/${file}`,
                         "utf8");

        var result = xml2json(file_xml, options);

        var json_file = `${file.split(".")[0]}.json`;
        var save_file = join(save_file_path, json_file);

        writeFileSync(save_file, result);


    }

}


function get_file_names(file_path) {

    var files = [];
    //console.log("getting file names...");
    readdirSync(file_path).forEach(file => {
        files.push(file)
        //console.log(files)

    })

    return files

}


function main() {

    var micro_file_path = "../xml_datasets/xml_micro";
    var mono_file_path = "../xml_datasets/xml_mono";

    var micro_save_path = "../results/js_results/json_micro";
    var mono_save_path = "../results/js_results/json_mono";


    var start_time = Date.now();
    convert_xml(micro_file_path, micro_save_path)
    var end_time = Date.now();
    var execution_time = (end_time - start_time) / 1000;
    console.log(`execution time ${execution_time} seconds`)
    appendFileSync("../results/benchmark.txt", `(javascript) time for converting micro files: ${execution_time} seconds \n`)
    
    // var start_time = Date.now();
    // convert_xml(mono_file_path, mono_save_path);
    // var end_time = Date.now();
    // var execution_time = (end_time - start_time) / 1000;
    // console.log(`execution time ${execution_time} seconds`)
    appendFileSync("../results/benchmark.txt", `(javascript) time for converting mono file: N/A (string size error) \n`)


}

main()
// node runs out of default memmory for large mono file
//  node --max-old-space-size=10000 js_xml_conversion.js