#!/bin/sh

cargo build --release
rm -r ~/.local/share/inventory_managoat
./target/release/inv ct -m 0.5 --open-by-default true "type 1"
./target/release/inv ct "type 2"
./target/release/inv rt
./target/release/inv ut 1 --name "type 1 - edited"
./target/release/inv dt 2
./target/release/inv rt
./target/release/inv ut 1 --ttl="1s"
./target/release/inv ci 1
./target/release/inv ci 1 --expires-at "2100-01-01 00:00:00"
./target/release/inv rt
echo "List Missing"
./target/release/inv list-missing
sleep 1.1
echo "Expired:"
./target/release/inv list-expired
./target/release/inv ri
./target/release/inv rt
echo "Use 0.5"
./target/release/inv use 1 0.5
./target/release/inv ri
./target/release/inv rt
echo "Use 1.8"
./target/release/inv use 1 1.8
./target/release/inv ri
./target/release/inv rt
echo "List Missing"
./target/release/inv list-missing
