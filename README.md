Support an Open Source Developer! :hearts:  

[![Become a patron](https://c5.patreon.com/external/logo/become_a_patron_button.png)](https://www.patreon.com/jojolepro)

# Inventory Managoat
A Simple Command Line Inventory Manager.  
Prevent food waste and never lose track of your items!

## Description
Use Inventory Manager to keep a list of things that you own.  
It provides simple but effective ways to interact with this list.  

To see all available options:
```sh
inv --help
inv <SUBCOMMAND> --help
```

## Common Usage

Create and print new item types:
```sh
# Creates the "Toilet Paper" type. You always want to keep at least 5 of those.
inv ct "Toilet Paper" --minimum-quantity 5
>0 # The ID for toilet paper
# Creates the "Milk" type which stays fresh one week.
inv ct "Milk" --ttl "1week"
>1 # The ID for milk
# Show the existing types.
inv rt
```

Create a new item instance (a specific item that exists):
```sh
# Creates 3 instances of toilet paper (ID = 0)
inv ci 0 --quantity 3
>2 # Instance ID for those three toilet paper rolls
# List instances
inv ri
```

List items that you don't have enough of:
```sh
inv list-missing
```

Use an item:
```sh
# Use a toilet paper instance (ID = 2)
inv use 2
```

Put an item to the trash:
```sh
# Trash a toilet paper instance (ID = 2)
inv trash 2
```

## Install From The AUR
If you have access to the Arch User Repository, you can install the package like this:
```sh
yay -S inv
```

## Install Using Cargo
First, install Rust (using [rustup](https://rustup.rs/)).
Then, it is as simple as:
```sh
cargo install -f inv
```

## Build From Source
First, install Rust (using [rustup](https://rustup.rs/)).

Then, run the following to build from source:
```sh
# Create a local copy
git clone https://github.com/jojolepro/inventory-managoat
cd inventory-managoat

# Build from source files
cargo build --release

# Install for the current user
mv target/release/inv ~/.local/bin/inv
```

Note: On windows, you should run those commands inside of [git bash](https://gitforwindows.org/) or [Windows Subsystem for Linux](https://docs.microsoft.com/en-us/windows/wsl/install-win10).

