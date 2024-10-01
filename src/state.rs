use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use serde::{Deserialize, Serialize};

pub enum ApiResponse {
    Ok,
    Created,
    JsonData(String),
}

impl IntoResponse for ApiResponse {
    fn into_response(self) -> Response {
        match self {
            Self::Ok => StatusCode::OK.into_response(),
            Self::Created => StatusCode::CREATED.into_response(),
            Self::JsonData(data) => (StatusCode::OK, data).into_response(),
        }
    }
}

#[derive(Debug)]
pub enum ApiError {
    BadRequest,
    Forbidden,
    Unauthorised,
    InternalServerError,
}

impl IntoResponse for ApiError {
    fn into_response(self) -> Response {
        // Convert your ApiError into an appropriate HTTP response
        // For example:
        match self {
            ApiError::BadRequest => ApiError::BadRequest.into_response(),
            ApiError::Forbidden => ApiError::Forbidden.into_response(),
            ApiError::Unauthorised => ApiError::Unauthorised.into_response(),
            ApiError::InternalServerError => ApiError::InternalServerError.into_response(),
        }
    }
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
#[serde(rename_all = "PascalCase")]
pub struct RyoState {
    pub job_progress: usize,
    pub job_qty: usize,
    pub node_levels: NodeLevels,
    pub node_weights: NodeWeights,
    pub status: Status,
    pub system_cleaning_maintenance: SystemCleaningMaintenance,
    #[serde(rename = "jobSetupStep")]
    pub job_setup_step: JobSetupStep,
}

#[derive(Serialize, Deserialize, Debug, Default)]
#[serde(rename_all = "camelCase")]
pub struct JobProgress {
    job_progress: usize,
}

#[derive(Serialize, Deserialize, Debug, Default, Clone)]
#[serde(rename_all = "UPPERCASE")]
pub enum NodeLevel {
    #[default]
    Low,
    Med,
    Loaded,
}

#[derive(Serialize, Deserialize, Debug, Default, Clone)]
#[serde(rename_all = "camelCase")]
pub struct NodeLevels {
    node_a_level: NodeLevel,
    node_a_level_ingredient_id: String,
    node_a_level_ingredient_name: String,
    node_b_level: NodeLevel,
    node_b_level_ingredient_id: String,
    node_b_level_ingredient_name: String,
    node_c_level: NodeLevel,
    node_c_level_ingredient_id: String,
    node_c_level_ingredient_name: String,
    node_d_level: NodeLevel,
    node_d_level_ingredient_id: String,
    node_d_level_ingredient_name: String,
}

#[derive(Serialize, Deserialize, Debug, Default, Clone)]
#[serde(rename_all = "camelCase")]
pub struct NodeWeights {
    node_a_weight: u32,
    node_b_weight: u32,
    node_c_weight: u32,
    node_d_weight: u32,
}

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub enum SystemStatus {
    PauseJob,
    RunJob,
    RunningJob,
    ReadyToStartJob,
    CancelJob,
    ResumeJob,
    #[default]
    StopSystem,
    PreparingJob,
    RefillNodesSystem,
    Faulted,
}

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[serde(rename_all = "UPPERCASE")]
enum DoorStatus {
    #[default]
    Open,
    Close,
}

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct Status {
    front_door_status: String,
    system_status: SystemStatus,
}

#[derive(Serialize, Deserialize, Debug, Default, Clone)]
#[serde(rename_all = "camelCase")]
pub struct SystemCleaningMaintenance {
    bag_management: String,
    clean_mode: String,
    front_door_control: String,
    side_door_control: String,
}

#[derive(Debug, Serialize, Deserialize, Default, Clone)]
#[serde(rename_all = "camelCase")]
pub struct StepDetails {
    confirm_configuration: String,
    load_detect: bool,
    position: i32,
    status: bool,
    step_description: String,
    step_id: String,
    step_tutorial: String,
    title: String,
    visible_check: bool,
}

#[derive(Debug, Serialize, Deserialize, Default, Clone)]
#[serde(rename_all = "UPPERCASE")]
pub struct JobSetupStep {
    bag_feeder: StepDetails,
    conveyors: StepDetails,
    conveyor_lids: StepDetails,
    remove_all_ingredient_sliders: StepDetails,
    take_system_photo: StepDetails,
    tunnels: StepDetails,
}

// mod tests {
//     use crate::state;
//
//     use crate::state::{NodeLevel, NodeLevels, NodeWeights, Status, SystemStatus};
//
//     #[test]
//     fn test_node_levels() {
//
//         use serde_json::json;
//         let levels = NodeLevels {
//             node_a_level: NodeLevel::Low,
//             node_a_level_ingredient_id: "1".to_string(),
//             node_b_level: NodeLevel::Low,
//             node_b_level_ingredient_id: "2".to_string(),
//             node_c_level: NodeLevel::Low,
//             node_c_level_ingredient_id: "3".to_string(),
//             node_d_level: NodeLevel::Low,
//             node_d_level_ingredient_id: "4".to_string(),
//         };
//
//         let levels_json = json!({
//         "nodeALevel": "LOW",
//         "nodeALevelIngredientId": "1",
//         "nodeBLevel": "LOW",
//         "nodeBLevelIngredientId": "2",
//         "nodeCLevel": "LOW",
//         "nodeCLevelIngredientId": "3",
//         "nodeDLevel": "LOW",
//         "nodeDLevelIngredientId": "4"
//     });
//
//         let levels_conv = serde_json::to_string(&levels).unwrap();
//         assert_eq!(levels_json.to_string(), levels_conv);
//     }
//
//     #[test]
//     fn test_node_weights() {
//         use serde_json::json;
//         let weights = NodeWeights {
//             node_a_weight: 200,
//             node_b_weight: 300,
//             node_c_weight: 400,
//             node_d_weight: 500,
//         };
//         let weights_json = json!({
//         "nodeAWeight": 200,
//         "nodeBWeight": 300,
//         "nodeCWeight": 400,
//         "nodeDWeight": 500,
//     });
//
//         let levels_conv = serde_json::to_string(&weights).unwrap();
//         assert_eq!(weights_json.to_string(), levels_conv);
//     }
//
//     #[test]
//     fn test_system_status() {
//         use state::DoorStatus;
//         use serde_json::json;
//         let status = Status {
//             front_door_status: DoorStatus::Open,
//             system_status: SystemStatus::PauseJob,
//         };
//         let status_json = json!({
//         "frontDoorStatus": "OPEN",
//         "systemStatus": "PauseJob",
//     });
//         assert_eq!(
//             status_json.to_string(),
//             serde_json::to_string(&status).unwrap()
//         );
//     }
// }
