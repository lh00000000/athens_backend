use std::fmt;

#[derive(Serialize, Deserialize, Debug)]
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

#[derive(Serialize, Deserialize, Debug)]
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

#[derive(Serialize, Deserialize, Debug)]
pub struct GazeAngle {
    x: f64,
    y: f64,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct GazeDirection0 {
    x: f64,
    y: f64,
    z: f64,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PoseEstimate {
    Rx: f64,
    Ry: f64,
    Rz: f64,
    Tx: f64,
    Ty: f64,
    Tz: f64,
}


pub type FaceId = i64;

#[derive(Serialize, Deserialize, Debug)]
pub struct Face {
    AU_c: AuC,
    AU_r: AuR,
    pub face_id: FaceId,
    frame_num: i64,
    gazeDirection0: GazeDirection0,
    gazeDirection1: GazeDirection0,
    gaze_angle: GazeAngle,
    landmark_confidence: f64,
    landmark_detection_success: bool,
    pose_estimate: PoseEstimate,
    pub time_stamp: f64,
}

#[derive(Hash, Eq, PartialEq, Debug, Clone)]
pub enum Personality {
    Open,
    Conscientious,
    Extroverted,
    Agreeable,
    Neurotic
}


impl fmt::Display for Personality {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[derive(Hash, Eq, PartialEq, Debug)]
pub enum Emotion {
    Happiness,
    Sadness,
    Surprise,
    Fear,
    Anger,
    Disgust,
    Contempt,
}

impl fmt::Display for Emotion {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl Face {
    pub fn personalities(&self) -> Vec<Personality> {

        let mut personalities = Vec::new();

        let emotion_personality_map = hashmap! {
            Emotion::Happiness => Personality::Open,

            Emotion::Sadness => Personality::Conscientious,
            Emotion::Disgust => Personality::Conscientious,

            Emotion::Surprise => Personality::Extroverted,

            Emotion::Fear => Personality::Agreeable,

            Emotion::Anger => Personality::Neurotic,
        };

        if self.AU_c.AU06 + self.AU_c.AU12 > 1.0 {
            personalities.push(emotion_personality_map.get(&Emotion::Happiness).unwrap().clone())
        }

        if self.AU_c.AU01 + self.AU_c.AU04 + self.AU_c.AU15 > 2.0 {
            personalities.push(emotion_personality_map.get(&Emotion::Sadness).unwrap().clone())
        }

        if self.AU_c.AU01 + self.AU_c.AU02 + self.AU_c.AU26 > 2.0 {
            personalities.push(emotion_personality_map.get(&Emotion::Surprise).unwrap().clone())
        }

        if self.AU_c.AU01 + self.AU_c.AU02 + self.AU_c.AU04 + self.AU_c.AU05 + self.AU_c.AU07 + self.AU_c.AU20 + self.AU_c.AU26 > 6.0 {
            personalities.push(emotion_personality_map.get(&Emotion::Fear).unwrap().clone())
        }

        if self.AU_c.AU04 + self.AU_c.AU05 + self.AU_c.AU07 + self.AU_c.AU23 > 3.0 {
            personalities.push(emotion_personality_map.get(&Emotion::Anger).unwrap().clone())
        }

        if self.AU_c.AU09 + self.AU_c.AU15 > 1.0 {
            personalities.push(emotion_personality_map.get(&Emotion::Disgust).unwrap().clone())
        }

        return personalities;
    }
}