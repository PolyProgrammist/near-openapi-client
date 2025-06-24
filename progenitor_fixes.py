import json
import re
import itertools
import sys
import os

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
    all_lib_rs_file = open('./near-openapi/src/lib.rs', 'r')
    lib_rs = all_lib_rs_file.read()
    all_lib_rs_file.close()
    
    types_start = """#[doc = r" Types used as operation parameters and responses."]
#[allow(clippy::all)]
pub mod types {"""
    types_index = lib_rs.find(types_start)
    client_index = lib_rs.find("""#[derive(Clone, Debug)]
#[doc = "Client for NEAR Protocol JSON RPC API""")
    
    print(types_index, client_index)
    
    dependencies = lib_rs[:types_index]
    types = lib_rs[types_index:client_index]
    client = lib_rs[client_index:]

    types = 'pub use near_account_id::AccountId;' + types[len(types_start):-2]
    account_id_start = types.find('#[doc = "NEAR Account Identifier')
    account_id_validity_start = types.find('#[doc = "`AccountIdValidityRulesVersion`"]')
    types = types[:account_id_start] + types[account_id_validity_start:]
    
    types_lib_rs = dependencies + types
    client_lib_rs = dependencies + client
    
    client_lib_rs = 'pub use near_openapi_types::*;\nuse near_openapi_types as types;\n' + client_lib_rs
    client_lib_rs = re.sub('"{}/\w*', '"{}/', client_lib_rs)

    if not os.path.isdir('./near-openapi-client/src'):
        os.makedirs('./near-openapi-client/src')
    client_lib_rs_file = open('./near-openapi-client/src/lib.rs', 'w')
    client_lib_rs_file.write(client_lib_rs)
    client_lib_rs_file.close()
    
    if not os.path.isdir('./near-openapi-types/src'):
        os.makedirs('./near-openapi-types/src')
    types_lib_rs_file = open('./near-openapi-types/src/lib.rs', 'w')
    types_lib_rs_file.write(types_lib_rs)
    types_lib_rs_file.close()
    
    all_cargo_toml_file = open('./near-openapi/Cargo.toml', 'r')
    cargo_toml = all_cargo_toml_file.read()
    all_cargo_toml_file.close()
    
    cargo_toml = re.sub('\[workspace\]', '', cargo_toml)
    
    client_cargo_toml = re.sub('near-openapi', 'near-openapi-client', cargo_toml)
    client_cargo_toml = re.sub('license = "SPECIFY A LICENSE BEFORE PUBLISHING"', """license = "MIT OR Apache-2.0"
repository = "https://github.com/polyprogrammist/near-openapi-client"
description = "Progenitor-generated client of NEAR JSON RPC API"
""", client_cargo_toml)
    client_cargo_toml += 'near-openapi-types = { path = "../near-openapi-types" }\n'
    types_cargo_toml = re.sub('near-openapi', 'near-openapi-types', cargo_toml)
    types_cargo_toml = re.sub('license = "SPECIFY A LICENSE BEFORE PUBLISHING"', """license = "MIT OR Apache-2.0"
repository = "https://github.com/polyprogrammist/near-openapi-client"
description = "Types for progenitor-generated client of NEAR JSON RPC API"
""", types_cargo_toml)
    types_cargo_toml += 'near-account-id = { version = "1.1.1", features = ["serde"] }\n'
    
    client_cargo_toml_file = open('./near-openapi-client/Cargo.toml', 'w')
    client_cargo_toml_file.write(client_cargo_toml)
    client_cargo_toml_file.close()
    
    types_cargo_toml_file = open('./near-openapi-types/Cargo.toml', 'w')
    types_cargo_toml_file.write(types_cargo_toml)
    types_cargo_toml_file.close()
    
    print('lib fixed')
