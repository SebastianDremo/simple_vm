#!/bin/bash

set -xe

test -f ./program.vm

cd ./vm_rust

cargo run 

cd ..

# dotnet run ./vm_dotnet/
