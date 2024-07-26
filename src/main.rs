pub mod dlog_proof;
use dlog_proof::*;
use k256::{ProjectivePoint, Scalar};
use std::time::Instant;

fn main() {
    let x: Scalar = generate_random();
    let g: ProjectivePoint = G;
    let y: ProjectivePoint = g * x;
    let sid: String = "sid".to_string();
    let pid: i32 = 1;

    let start_proof: Instant = Instant::now();
    let dlog_proof: DLogProof = DLogProof::prove(sid.clone(), pid, x, y, g);
    println!(
        "Proof computation time: {}ms",
        start_proof.elapsed().as_millis()
    );

    println!("");
    // unfortunately the x, y coordinates are private fields both in ProjectivePoint and AffinePoint structs
    // so i have to print it that way, this is a difference between this Rust implementation and the original script.
    println!("t: {}", dlog_proof.point_to_hex(&dlog_proof.t));
    println!("s: {}", dlog_proof.scalar_to_hex(&dlog_proof.s));
    println!("");

    let start_verify = Instant::now();
    let result = dlog_proof.verify(sid, pid, y, g);
    println!(
        "Verify computation time: {}ms",
        start_verify.elapsed().as_millis()
    );

    if result {
        println!("DLOG proof is correct");
    } else {
        println!("DLOG proof is not correct");
    }
}
