use std::fmt;
use std::fs::File;
use std::io::Read;
use std::collections::HashMap;
use std::collections::HashSet;

// macro that expands into a bunch of ifs that check if any of the fields are None
macro_rules! zoom_and_enhance {
    ($(#[$struct_meta:meta])*
    pub struct $name:ident { $($fname:ident : $ftype:ty),* }) => {
        $(#[$struct_meta])*
        pub struct $name {
            $($fname: $ftype),*
        }

        impl $name {
            pub fn positive_aucs(&self) -> Vec<String> {
                let mut positive_aucs: Vec<String> = Vec::new();
                // here's the expansion
                $(
                if self.$fname >= 1.0 {
                    positive_aucs.push(stringify!($fname).to_string());
                };
                )*
                positive_aucs
            }

            pub fn auc(&self, auc_str: &str) -> f64 {
                $(
                if stringify!($fname) == auc_str {
                    return self.$fname;
                }
                )*
                0.0
            }
        }
    };
}

zoom_and_enhance! {
    #[allow(non_snake_case)]
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
        AU45: f64
    }
}

#[allow(non_snake_case)]
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
pub struct GazeDirection {
    x: f64,
    y: f64,
    z: f64,
}

#[allow(non_snake_case)]
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

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize, Debug)]
pub struct Face {
    AU_c: AuC,
    AU_r: AuR,
    pub face_id: FaceId,
    frame_num: i64,
    gazeDirection0: GazeDirection,
    gazeDirection1: GazeDirection,
    gaze_angle: GazeAngle,
    landmark_confidence: f64,
    landmark_detection_success: bool,
    pose_estimate: PoseEstimate,
    pub time_stamp: f64,
}



macro_rules! variant_getter {
    ($(#[$struct_meta:meta])*
    pub enum $name:ident { $($fname:ident),* }) => {
        $(#[$struct_meta])*
        pub enum $name {
            $($fname),*
        }

        impl $name {
            pub fn variant(var: &str) -> Self {
                $(
                if stringify!($fname) == var {
                    return $name::$fname;
                }
                )*
                $name::default()

            }
        }
    };
}

variant_getter! {
    #[derive(Hash, Eq, PartialEq, Debug, Clone)]
    pub enum Personality {
        Open,
        Conscientious,
        Extroverted,
        Agreeable,
        Neurotic
    }
}

impl Personality {
    pub fn default() -> Self {
        Personality::Open
    }
}

impl fmt::Display for Personality {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

variant_getter! {
    #[derive(Hash, Eq, PartialEq, Debug)]
    pub enum Emotion {
        Happiness,
        Sadness,
        Surprise,
        Fear,
        Anger,
        Disgust,
        Contempt
    }
}

impl Emotion {
    pub fn default() -> Self {
        Emotion::Happiness
    }
}

impl fmt::Display for Emotion {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[derive(Debug, Serialize, Deserialize)]
struct AucEmotionPersonality {
    emotion_personality: HashMap<String, String>,
    auc_emotion: HashMap<String, Vec<String>>,
}

impl Face {
    pub fn new() -> Self {
        Face {
            AU_c: AuC {
                AU01: 1.0,
                AU02: 0.0,
                AU04: 0.0,
                AU05: 0.0,
                AU06: 1.0,
                AU07: 0.0,
                AU09: 0.0,
                AU10: 0.0,
                AU12: 1.0,
                AU14: 0.0,
                AU15: 5.0,
                AU17: 0.0,
                AU20: 0.0,
                AU23: 0.0,
                AU25: 1.0,
                AU26: 0.0,
                AU28: 0.0,
                AU45: 7.0,
            },
            AU_r: AuR {
                AU01: 0.0,
                AU02: 0.0,
                AU04: 0.0,
                AU05: 0.0,
                AU06: 0.0,
                AU07: 0.0,
                AU09: 0.0,
                AU10: 0.0,
                AU12: 0.0,
                AU14: 0.0,
                AU15: 0.0,
                AU17: 0.0,
                AU20: 0.0,
                AU23: 0.0,
                AU25: 0.0,
                AU26: 0.0,
                AU45: 0.0,
            },
            face_id: 0,
            frame_num: 0,
            gazeDirection0: GazeDirection {
                x: 0.0,
                y: 0.0,
                z: 0.0,
            },
            gazeDirection1: GazeDirection {
                x: 0.0,
                y: 0.0,
                z: 0.0,
            },
            gaze_angle: GazeAngle { x: 0.0, y: 0.0 },
            landmark_confidence: 0.0,
            landmark_detection_success: false,
            pose_estimate: PoseEstimate {
                Rx: 0.0,
                Ry: 0.0,
                Rz: 0.0,
                Tx: 0.0,
                Ty: 0.0,
                Tz: 0.0,
            },
            time_stamp: 0.0,
        }
    }

    pub fn personality(&self) -> Option<Personality> {
        let mut file = File::open("auc_emotion_personality_map.json").unwrap();
        let mut data = String::new();
        file.read_to_string(&mut data).unwrap();
        let auc_emotion_personality_map = serde_json::from_str::<AucEmotionPersonality>(&data).unwrap();

        let mut positive_aucs = HashSet::new();
        for auc in self.AU_c.positive_aucs() {
            positive_aucs.insert(auc);
        }

        for (emotion, aucs) in &auc_emotion_personality_map.auc_emotion {
            let mut aucs_set = HashSet::new();
            for auc in aucs {
                aucs_set.insert(auc.clone());
            }
            if aucs_set.is_subset(&positive_aucs) {
                match auc_emotion_personality_map.emotion_personality.get(emotion) {
                    Some(p) => return Some(Personality::variant(p)),
                    None => return None
                }
            }
        }
        None
    }
}