#!/bin/bash

set -e

test -f ./program.vm

cd ./vm_rust

cargo run 

cd ..

echo Done

# dotnet run ./vm_dotnet/
