extern crate k256;
extern crate rand;
extern crate sha2;

use k256::elliptic_curve::sec1::ToEncodedPoint;
use k256::elliptic_curve::PrimeField;
use k256::{AffinePoint, ProjectivePoint, Scalar};
use rand::rngs::OsRng;
use sha2::{Digest, Sha256};
use std::fmt;

pub const G: ProjectivePoint = ProjectivePoint::GENERATOR;

pub fn generate_random() -> Scalar {
    Scalar::generate_vartime(&mut OsRng)
}

#[derive(Debug, Clone)]
pub struct DLogProof {
    t: ProjectivePoint,
    s: Scalar,
}

// Associated constants and methods
impl DLogProof {
    fn new(t: ProjectivePoint, s: Scalar) -> Self {
        DLogProof { t, s }
    }

    pub fn hash_points(sid: String, pid: i32, points: Vec<ProjectivePoint>) -> Scalar {
        let mut h = Sha256::new();
        h.update(sid.as_bytes());
        h.update(pid.to_be_bytes());
        for p in points {
            let affine_point: AffinePoint = p.into();
            // usually AffinePoint are compressed according to the docs
            h.update(affine_point.to_encoded_point(true).as_bytes())
        }
        let digest = h.finalize();
        println!("Hash: {:x}", digest);
        let scalar = Scalar::from_repr(digest);
        println!("{:?}", scalar);
        scalar.unwrap()
    }

    pub fn prove(
        sid: String,
        pid: i32,
        x: Scalar,
        y: ProjectivePoint,
        base_point: ProjectivePoint,
    ) -> DLogProof {
        let r = generate_random();
        let t = base_point * r;
        let c = DLogProof::hash_points(sid, pid, vec![base_point, y, t]);

        let s = r + c * x;
        DLogProof::new(t, s)
    }

    pub fn verify(
        &self,
        sid: String,
        pid: i32,
        y: ProjectivePoint,
        base_point: ProjectivePoint,
    ) -> bool {
        let c = Self::hash_points(sid, pid, vec![base_point, y, self.t]);
        let lhs = base_point * self.s;
        let rhs = self.t + (y * c);
        lhs == rhs
    }
}

// Implement equality comparison
impl PartialEq for DLogProof {
    fn eq(&self, other: &Self) -> bool {
        self.t == other.t && self.s == other.s
    }
}

// Implement the `Eq` trait as well
impl Eq for DLogProof {}

// Implement Display for pretty printing
impl fmt::Display for DLogProof {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "DLogProof {{ t: {:?}, s: {:?} }}", self.t, self.s)
    }
}

trait ToBytes {
    fn to_bytes(&self) -> Vec<u8>;
}

impl ToBytes for ProjectivePoint {
    fn to_bytes(&self) -> Vec<u8> {
        let affine_point: AffinePoint = (*self).into();
        affine_point.to_encoded_point(true).as_bytes().to_vec()
    }
}
