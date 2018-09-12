use bincode::{serialize, deserialize};

#[derive(Serialize, Deserialize)]
pub struct AuC {
    AU01: f64,
    AU02: f64,
    AU04: f64,
    AU05: f64,
    AU06: f64,
    AU07: f64,
    AU09: f64,
    AU10: f64,
    AU12: f64,
    AU14: f64,
    AU15: f64,
    AU17: f64,
    AU20: f64,
    AU23: f64,
    AU25: f64,
    AU26: f64,
    AU28: f64,
    AU45: f64,
}

#[derive(Serialize, Deserialize)]
pub struct AuR {
    AU01: f64,
    AU02: f64,
    AU04: f64,
    AU05: f64,
    AU06: f64,
    AU07: f64,
    AU09: f64,
    AU10: f64,
    AU12: f64,
    AU14: f64,
    AU15: f64,
    AU17: f64,
    AU20: f64,
    AU23: f64,
    AU25: f64,
    AU26: f64,
    AU45: f64,
}

#[derive(Serialize, Deserialize)]
pub struct GazeAngle {
    x: f64,
    y: f64,
}

#[derive(Serialize, Deserialize)]
pub struct GazeDirection0 {
    x: f64,
    y: f64,
    z: f64,
}

#[derive(Serialize, Deserialize)]
pub struct PoseEstimate {
    Rx: f64,
    Ry: f64,
    Rz: f64,
    Tx: f64,
    Ty: f64,
    Tz: f64,
}

#[derive(Serialize, Deserialize)]
pub struct Face {
    AU_c: AuC,
    AU_r: AuR,
    face_id: i64,
    frame_num: i64,
    gazeDirection0: GazeDirection0,
    gazeDirection1: GazeDirection0,
    gaze_angle: GazeAngle,
    landmark_confidence: f64,
    landmark_detection_success: bool,
    pose_estimate: PoseEstimate,
    time_stamp: f64,
}


impl Face {
    pub fn as_vec_u8(&self) -> Vec<u8> {
        return serialize(&self).unwrap();
    }
    pub fn from_vec_u8(v: &[u8]) -> Face {
        return deserialize(v).unwrap()
    }

}