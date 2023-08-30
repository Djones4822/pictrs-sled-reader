## Sled Reader

Rust script to dump a sled database to plain text files. Iterates over each tree and converts the key and values within the tree to strings
and dumps them into a .txt file inside of ./text_dumps 

## Usage
 - make sure rust is installed with cargo
    - curl https://sh.rustup.rs -sSf | sh
 - copy the entire sled db folder to this directory, or alter the line to point to the sled directory. 
 - install the sled cargo 
     - cargo add sled
 - execute the script from the top level of the repo
     -  cargo run

 Files will be put into the ./text_dumps 
