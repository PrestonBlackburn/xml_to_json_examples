import json
import os
import time
import functools

import xmltodict



def timer(func):
    # save the execution time
    @functools.wraps(func)
    def wrapper_timer(*args, **kwargs):
        start_time = time.perf_counter()
        value = func(*args, **kwargs)
        end_time = time.perf_counter()
        run_time = end_time - start_time
        record_string = f"(python) time for {kwargs['load_file_path']}: {run_time} seconds\n"

        with open("results/benchmark.txt", 'a') as file:
            file.write(record_string)
        return value
    
    return wrapper_timer


@timer
def convert_xml(load_file_path:str = "",
                save_file_path:str = ""
                ) -> None:
    # read the files, convert the files, save the files
    
    files = get_file_names(load_file_path)

    for file_name in files:
        with open(f'{load_file_path}/{file_name}', 'r') as file:
            tree_text = file.read()

        testing_xml_parsed = xmltodict.parse(tree_text)

        json_string = json.dumps(testing_xml_parsed)

        json_file = file_name.split(".")[0] + ".json"

        with open(f"{save_file_path}/{json_file}", 'w') as file:
            file.write(json_string)

    return


def get_file_names(file_path:str) -> list:
    files = []
    for (dirpath, dirnames, filenames) in os.walk(file_path):
        if "checkpoint" not in filenames:
            files.extend(filenames)

    return files





if __name__ == "__main__":

    # run from "xml_to_json_benchmarks" level

    micro_file_path = "xml_datasets/xml_micro"
    mono_file_path = "xml_datasets/xml_mono"

    micro_save_path = "results/python_results/json_micro"
    mono_save_path = "results/python_results/json_mono"
    
    
    convert_xml(load_file_path = micro_file_path,
                save_file_path = micro_save_path)
    
    convert_xml(load_file_path = mono_file_path,
                save_file_path = mono_save_path)
    


