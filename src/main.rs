pub mod dlog_proof;
use dlog_proof::*;
use std::time::Instant;

fn main() {
    let x = generate_random();
    let g_projective = G;
    let y = g_projective * x;
    let sid: String = "sid".to_string();
    let pid: i32 = 1;

    let start_proof = Instant::now();
    let dlog_proof = DLogProof::prove(sid.clone(), pid, x, y, g_projective);
    println!(
        "Proof computation time: {}ms",
        start_proof.elapsed().as_millis()
    );

    println!("{}", dlog_proof.point_to_hex(&dlog_proof.t));
    dlog_proof.print_t_coordinates();
    // will need to convert t to and affinepoint to get access to the x and y coordinates
    // like in dlog_proof.py

    // println!("");
    // println!("{} {}", dlog_proof.t);
    /*print(dlog_proof.t.x(), dlog_proof.t.y())
    print(dlog_proof.s) */
    // println!("");

    let start_verify = Instant::now();
    let result = dlog_proof.verify(sid, pid, y, g_projective);
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
