
# About

This project uses a mix of rust and c, with a c interface to the linux kernel proc connector that receives notifications whenever a process starts or exits. This is used in the rust daemon that grabs some of the process info and simply spits it out as a log to stdout or a file.
You might need to be root to get results. From my testing I didn't require root but my user is also In like 20 supplementary groups that might have given me access.

# Install

## Requirements

rustc (stable), gcc, make
No external libraries are required.

## Instructions

Use `make proctrackd` to make the daemon.

# Usage

Try `proctrackd -h` for command line help.
