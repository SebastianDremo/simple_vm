#!/bin/bash

set -e

test -f ./program.vm

cd ./vm_rust

cargo run 

cd ..

# dotnet run ./vm_dotnet/
