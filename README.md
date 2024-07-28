# ZK DLOG proof

The code is the adaptation of a script to perform the core concepts of creating a zero knowledge discrete logarithm proof using the Schnorr protocol with Fiat-Shamir transformation.
## Usage
To execute the program and pass to it the sid and pid argument run the command:
```sh
cargo run -- --sid yoursid --pid 1337
```
In case the sid argument contains space, use double quotes like so:
```sh
cargo run -- --sid "hire me please" --pid 1337
```
You can also execute the program without arguments, the default sid is "sid" and default pid is 1.
