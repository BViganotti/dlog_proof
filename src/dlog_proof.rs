extern crate k256;
extern crate rand;
extern crate sha2;

use generic_array::typenum::U32;
use k256::elliptic_curve::group::GroupEncoding;
use k256::elliptic_curve::sec1::ToEncodedPoint;
use k256::elliptic_curve::PrimeField;
use k256::{AffinePoint, CompressedPoint, ProjectivePoint, Scalar};
use rand::rngs::OsRng;
use serde_json;
use sha2::digest::generic_array::{self, GenericArray};
use sha2::{Digest, Sha256};

pub const G: ProjectivePoint = ProjectivePoint::GENERATOR;

pub fn generate_random() -> Scalar {
    Scalar::generate_vartime(&mut OsRng)
}

#[derive(Debug, Clone)]
pub struct DLogProof {
    pub t: ProjectivePoint,
    pub s: Scalar,
}

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
            // usually AffinePoint are compressed according to the docs, leaving it as true
            h.update(affine_point.to_encoded_point(true).as_bytes())
        }
        let digest = h.finalize();
        let scalar = Scalar::from_repr(digest);
        scalar.unwrap()
    }

    pub fn prove(
        sid: String,
        pid: i32,
        x: Scalar,
        y: ProjectivePoint,
        base_point: ProjectivePoint,
    ) -> DLogProof {
        let r: Scalar = generate_random();
        let t: ProjectivePoint = base_point * r;
        let c: Scalar = DLogProof::hash_points(sid, pid, vec![base_point, y, t]);
        // no need to do the mod operation like in python because the Scalar type and elliptic curve operations
        // are designed to work within the finite field defined by the curve’s order
        let s: Scalar = r + c * x;
        DLogProof::new(t, s)
    }

    pub fn verify(
        &self,
        sid: String,
        pid: i32,
        y: ProjectivePoint,
        base_point: ProjectivePoint,
    ) -> bool {
        let c: Scalar = Self::hash_points(sid, pid, vec![base_point, y, self.t]);
        let lhs: ProjectivePoint = base_point * self.s;
        let rhs: ProjectivePoint = self.t + (y * c);
        lhs == rhs
    }

    pub fn to_str(&self) -> String {
        serde_json::to_string(&self.to_dict()).unwrap()
    }

    pub fn to_dict(&self) -> serde_json::Map<String, serde_json::Value> {
        let mut map = serde_json::Map::new();
        map.insert(
            "t".to_string(),
            serde_json::Value::String(self.point_to_hex(&self.t)),
        );
        map.insert(
            "s".to_string(),
            serde_json::Value::String(self.scalar_to_hex(&self.s)),
        );
        map
    }

    pub fn from_dict(data: &serde_json::Map<String, serde_json::Value>) -> Self {
        let t_hex: &str = data.get("t").unwrap().as_str().unwrap();
        let s_hex: &str = data.get("s").unwrap().as_str().unwrap();

        let t_bytes: Vec<u8> = hex::decode(t_hex).unwrap();
        let s_bytes: Vec<u8> = hex::decode(s_hex).unwrap();

        let compressed: CompressedPoint = *CompressedPoint::from_slice(t_bytes.as_slice());
        let t: ProjectivePoint = ProjectivePoint::from_bytes(&compressed).unwrap();

        let tmp: GenericArray<u8, U32> = *GenericArray::from_slice(s_bytes.as_slice());
        let s: Scalar = Scalar::from_repr(tmp).unwrap();

        DLogProof { t, s }
    }

    pub fn from_str(json: &str) -> Self {
        let data: serde_json::Map<String, serde_json::Value> = serde_json::from_str(json).unwrap();
        Self::from_dict(&data)
    }

    // Scalars, ProjectivePoints and AffinePoints do provide some information when printed but not very usable info, hex seems like the only reasonable way to do it
    pub fn point_to_hex(&self, point: &ProjectivePoint) -> String {
        hex::encode(point.to_encoded_point(true).as_bytes())
    }

    pub fn scalar_to_hex(&self, scalar: &Scalar) -> String {
        hex::encode(scalar.to_bytes())
    }

    pub fn serialize(&self) -> serde_json::Map<String, serde_json::Value> {
        self.to_dict()
    }

    pub fn deserialize(&self, input: &serde_json::Map<String, serde_json::Value>) -> Self {
        Self::from_dict(input)
    }
}

impl PartialEq for DLogProof {
    fn eq(&self, other: &Self) -> bool {
        self.t == other.t && self.s == other.s
    }

    fn ne(&self, other: &Self) -> bool {
        !self.eq(other)
    }
}
