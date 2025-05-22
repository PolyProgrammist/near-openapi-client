import json
import re
import itertools
import sys

def reconstructAllOfOneOf(schema):
    # return
    print('reconstructing allOf to oneOf. only for testing with progenitor')
    all_of = schema["allOf"]
    
    # Check if all elements in allOf have oneOf
    one_of_lists = []
    for item in all_of:
        if "oneOf" in item:
            one_of_lists.append(item["oneOf"])
        else:
            # If there's an element in allOf that doesn't have oneOf,
            # we'll treat it as a oneOf with a single element
            one_of_lists.append([item])
    
    # Generate all combinations of the elements from all oneOf arrays
    combinations = list(itertools.product(*one_of_lists))
    
    # Create a new oneOf array with allOf entries for each combination
    new_one_of = []
    for combo in combinations:
        combined = {
            "allOf": list(combo)
        }
        new_one_of.append(combined)
    
    # Replace the allOf with oneOf in the schema
    schema.pop("allOf")
    schema["oneOf"] = new_one_of

def iterate_nested_json_for_loop(json_obj):
    if isinstance(json_obj, dict):
        if 'allOf' in json_obj:
            oneOfs = 0
            for item in json_obj['allOf']:
                if 'oneOf' in item:
                    oneOfs += 1
            if oneOfs >= 2:
                reconstructAllOfOneOf(json_obj)
        for key, value in json_obj.items():
            iterate_nested_json_for_loop(value)
    if isinstance(json_obj, list):
        for item in json_obj:
            iterate_nested_json_for_loop(item)

filename = 'openapi.json'

f = open(filename, 'r')
spec = json.load(f)
f.close()


if len(sys.argv) == 2 and sys.argv[1] == '--spec-fix':
    iterate_nested_json_for_loop(spec)
    print('spec fixed')

f = open(filename, 'w')
json.dump(spec, f, indent=4)
f.close()

if len(sys.argv) == 2 and sys.argv[1] == '--lib-fix':
    filename = './near-openapi-client/src/lib.rs'
    f = open(filename, 'r')
    filedata = f.read()
    f.close()

    newfiledata = re.sub('"{}/\w*', '"{}/', filedata)

    f = open(filename, 'w')
    f.write(newfiledata)
    f.close()
    
    print('lib fixed')


# cd testokplain && cargo run > transaction.json && cd ../progenitor && python3 tx.py && cargo progenitor -i ../testokplain/transaction.json -o keeper -n keeper -v 0.1.0 && python3 tx.py && cd user && cargo run && cd ../..