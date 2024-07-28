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

The output should resemble something like this:
```sh
sid: hire me please, pid: 1337
Proof computation time: 0ms

t: 0397ad7169d5fb6f32b3669ded129e336586d0d177ea8d8dd02e2444a8f01e430b
s: 910f237020e5614dac5086a965cc215a820a92aedc7698ccfd984aed29b26a7c

Verify computation time: 1ms
DLOG proof is correct

Below is a simple serialization and deserialization test:
serialized:
{"s": String("910f237020e5614dac5086a965cc215a820a92aedc7698ccfd984aed29b26a7c"), "t": String("0397ad7169d5fb6f32b3669ded129e336586d0d177ea8d8dd02e2444a8f01e430b")}

deserialized:
DLogProof { t: ProjectivePoint { x: FieldElement(FieldElementImpl { value: FieldElement5x52([1201392280552203, 2109588494942946, 736893537276113, 1956216782315217, 166771187897851]), magnitude: 1, normalized: true }), y: FieldElement(FieldElementImpl { value: FieldElement5x52([2404385006653081, 4501564259516388, 679251975903580, 4491590831166809, 120504777576134]), magnitude: 1, normalized: true }), z: FieldElement(FieldElementImpl { value: FieldElement5x52([1, 0, 0, 0, 0]), magnitude: 1, normalized: true }) }, s: Scalar(UInt { limbs: [Limb(18273437870523050620), Limb(9370463254418462924), Limb(12416572234775929178), Limb(10452612224645423437)] }) }
```