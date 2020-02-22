# Inventory Managoat
A Simple Command Line Based Inventory Manager.

## Basic Usage
inventory_managoat [FLAGS] [OPTIONS] \<SUBCOMMAND\>

## Author
This software was written by Jojolepro. Documentation was reviewed by Nikekson.
Support my work on [Patreon](https://www.patreon.com/jojolepro)!

## DESCRIPTION
Inventory Managoat is a command-line based inventory manager.
It is used to keep a list of "things" (usually physical items) that you own.
It provides simple but effective ways to interact with this list.

To use the program, you have to specify the general flags [FLAGS] and options [OPTIONS],
then the ACTION you want to execute over the inventory list \<SUBCOMMAND\>
and finally the options specific to that subcommand.

For a full usage description, use
```
inventory_managoat --help
inventory_managoat \<SUBCOMMAND\> --help
```
which will show all available options. Alternatively, read the rest of this page, which contains all the non subcommand-specific options.

## OPTIONS
```
--h, --help
Prints help information

--i, --interactive
Enables interactive mode using curses. TODO

--m, --minimal
Enables printing of the data without creating pretty tables. TODO

--q, --quiet
Enables quiet mode. Disables all output

--V, --version
Prints version information

--f, --fields \<fields\>...
Specify which fields should be printed. TODO

--n, --name \<name\>
Uses the inventory with this name. The files will be loaded and saved using this prefix. Defaults to "inventory".

--w, --workdir \<workdir\>
The directory to use to load and save the inventory files. Defaults to the default configuration directory of your user
```

## COMMANDS - Types

```
-ct
Create a new item type
-rt
Print one or multiple item type data
-ut
Modify the properties of an item type
-dt
Delete an item type
```

## COMMANDS - Instances

```
-ci
Create a new item instance
-ri
Print one or multiple item instance data
-ui
Modify the properties of an item instance
-di
Delete an item instance permanently and all records of it
```

## COMMANDS - Utilities

```
-list-expired
List expired item instances
-list-missing
List item types that do not have enough item instances to satisfy their minimum quantity
-trash
Put an item instance in the trash, keeping a record of its existence
-use
Use some quantity from an item type
```

## CUSTOMIZATION
Inventory Managoat is customized by specifying command line options or modifying the source code/patching in features according to your needs.

## ISSUES
See https://github.com/jojolepro/inventory-managoat/issues

## BUGS
Send all bug reports and pull requests/patches to https://github.com/jojolepro/inventory-managoat

