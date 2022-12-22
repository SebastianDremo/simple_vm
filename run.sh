#!/bin/bash

#set -e //needed in order to print VM exit code at the end

test -f ./program.vm

cd ./vm_rust

cargo run 

rust_exit_code=$?
if [ $rust_exit_code == 0 ];
then 
    cd ..
    exit 0
else 
    echo Program finished with $rust_exit_code code.
fi

cd ..

# dotnet run ./vm_dotnet/
