## Pictrs Sled Reader - Aliases and Delete Tokens Only

Rust script to dump the sled database tree "pict-rs-alias-delete-tokens-tree' used in pict-rs to plain text files. 

Iterates the tree and converts the key and values within the tree to strings
and dumps them into a .txt file inside of ./text_dumps 

## Usage
 - make sure rust is installed with cargo
    - curl https://sh.rustup.rs -sSf | sh
 - copy the entire sled db folder to this directory, or alter the line to point to the sled directory. 
 - install the sled cargo 
     - cargo add sled
 - execute the script from the top level of the repo
     -  cargo run

 Files will be put into the ./text_dumps folder inside this project

## Credits
The pict-rs repo is owned by the asonix team, certain parts of the code are reproduced here in order for the script to function. All code remains theirs and is licensed using their GNU copyleft license. What is reproduced here is slightly altered to accomodate what I was trying to accomplish. 

## License
This repository is licensed under the same license that pict-rs is, please see the LICENSE file for details. 