# CLI

## Build instructions
`cargo build` should be all you need and it will be in `target/debug/cli`

## Env file

1. `PLAYBOOK_DIR` for playbook dir
2. `INVENTORY_DIR` for inventory dir
3. `<playbook_name>.yaml` for env variables

### Example

``` sh
PLAYBOOK_DIR=../playbooks/utilities
INVENTORY_DIR=../playbooks/inventory.yaml
test_yaml="print_msg=hi,another_env=si"
```

## Functions

### List

#### Regular

Just list the names of all the playbooks

##### Examples
1. `cli list`
2. `cli -p playbooks/ -i inventory.yaml list`

#### Verbose

List the names of all the playbooks and their short descriptions as well

##### Examples
1. `cli -v list`

### Describe

#### Regular

Show the short description for the playbook inputted

##### Examples
1. `cli describe test.yaml`
2. `cli describe 0`
3. `cli describe 0-4`
4. `cli describe 1-2 test.yaml`

#### Verbose

Show the entire playbook inputted 

##### Examples
1. `cli -v describe test.yaml`

### Run

#### Regular

Just show success and fail depending on what happens

##### Examples
1. `cli run test.yaml`
2. `cli run test.yaml,print_msg=hi`
3. `cli run test.yaml,print_msg=hi,env2=something`

#### Verbose

Give the full output while running the playbook

##### Examples
1. `cli -v run test.yaml`
