# Rollviz

Rolling game dev tool for image visualization

## Install

Install rust with:

    curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
    
Build Rollviz tool:

    cargo build

## Use

For image:

    target/debug/rollviz image /home/bux/Images/rolling/testplage.png

For tile:

    target/debug/rollviz tile /home/bux/Images/rolling/test_herbe.png
