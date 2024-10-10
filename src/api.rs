use crate::state::ApiResponse::JsonData;
use crate::state::{
    ApiError, ApiResponse, JobSetupStep, NodeLevels, NodeWeights, RyoState, Status,
    SystemCleaningMaintenance,
};
use axum::extract::State;
use axum::Json;
use std::sync::Arc;
use tokio::sync::Mutex;

pub async fn get_state(State(state): State<Arc<Mutex<RyoState>>>) -> Result<ApiResponse, ApiError> {
    let ryo_state = state.lock().await;
    Ok(JsonData(serde_json::to_string(&*ryo_state).unwrap()))
}

pub async fn update_state(
    State(state): State<Arc<Mutex<RyoState>>>,
    Json(new_state): Json<RyoState>,
) -> Result<ApiResponse, ApiError> {
    let mut ryo_state = state.lock().await;
    *ryo_state = new_state;
    Ok(ApiResponse::Ok)
}

pub async fn get_job_progress(
    State(state): State<Arc<Mutex<RyoState>>>,
) -> Result<ApiResponse, ApiError> {
    let job_progress = &state.lock().await.job_progress;
    Ok(JsonData(serde_json::to_string(job_progress).unwrap()))
}

pub async fn get_node_levels(
    State(state): State<Arc<Mutex<RyoState>>>,
) -> Result<ApiResponse, ApiError> {
    println!("get node levels called");
    let node_levels = &state.lock().await.node_levels;
    Ok(JsonData(serde_json::to_string(node_levels).unwrap()))
}

pub async fn update_node_levels(
    State(state): State<Arc<Mutex<RyoState>>>,
    Json(new_state): Json<NodeLevels>,
) -> Result<ApiResponse, ApiError> {
    println!("{:?}", new_state);
    let mut ryo_state = state.lock().await;
    //ryo_state.node_levels = new_state;
    ryo_state.node_levels.node_a_level_ingredient_id = new_state.node_a_level_ingredient_id;
    ryo_state.node_levels.node_a_level_ingredient_name = new_state.node_a_level_ingredient_name;
    ryo_state.node_levels.node_b_level_ingredient_id = new_state.node_b_level_ingredient_id;
    ryo_state.node_levels.node_b_level_ingredient_name = new_state.node_b_level_ingredient_name;
    ryo_state.node_levels.node_c_level_ingredient_id = new_state.node_c_level_ingredient_id;
    ryo_state.node_levels.node_c_level_ingredient_name = new_state.node_c_level_ingredient_name;
    ryo_state.node_levels.node_d_level_ingredient_id = new_state.node_d_level_ingredient_id;
    ryo_state.node_levels.node_d_level_ingredient_name = new_state.node_d_level_ingredient_name;
    Ok(ApiResponse::Ok)
}

pub async fn get_node_weights(
    State(state): State<Arc<Mutex<RyoState>>>,
) -> Result<ApiResponse, ApiError> {
    println!("get node weights called");
    let node_levels = &state.lock().await.node_weights;
    Ok(JsonData(serde_json::to_string(node_levels).unwrap()))
}

pub async fn update_node_weight(
    State(state): State<Arc<Mutex<RyoState>>>,
    Json(new_weights): Json<NodeWeights>,
) -> Result<ApiResponse, ApiError> {
    let mut ryo_state = state.lock().await;
    ryo_state.node_weights = new_weights;
    Ok(ApiResponse::Ok)
}

pub async fn get_status(
    State(state): State<Arc<Mutex<RyoState>>>,
) -> Result<ApiResponse, ApiError> {
    let node_levels = &state.lock().await.status;
    Ok(JsonData(serde_json::to_string(node_levels).unwrap()))
}

pub async fn update_status(
    State(state): State<Arc<Mutex<RyoState>>>,
    Json(new_status): Json<Status>,
) -> Result<ApiResponse, ApiError> {
    let mut ryo_state = state.lock().await;
    ryo_state.status.system_status = new_status.system_status;
    Ok(ApiResponse::Ok)
}

pub async fn get_sys_clean_maint(
    State(state): State<Arc<Mutex<RyoState>>>,
) -> Result<ApiResponse, ApiError> {
    let sys_cleaning_maint = &state.lock().await.system_cleaning_maintenance;
    Ok(JsonData(serde_json::to_string(sys_cleaning_maint).unwrap()))
}

pub async fn update_sys_cleaning_maintenance(
    State(state): State<Arc<Mutex<RyoState>>>,
    Json(new_system_cleaning_maintenance): Json<SystemCleaningMaintenance>,
) -> Result<ApiResponse, ApiError> {
    let mut ryo_state = state.lock().await;
    ryo_state.system_cleaning_maintenance = new_system_cleaning_maintenance;
    Ok(ApiResponse::Ok)
}
pub async fn get_job_setup(
    State(state): State<Arc<Mutex<RyoState>>>,
) -> Result<ApiResponse, ApiError> {
    let job_setup_steps = &state.lock().await.job_setup_step;
    Ok(JsonData(serde_json::to_string(job_setup_steps).unwrap()))
}

pub async fn update_job_setup(
    State(state): State<Arc<Mutex<RyoState>>>,
    Json(new_job_setup): Json<JobSetupStep>,
) -> Result<ApiResponse, ApiError> {
    let mut ryo_state = state.lock().await;
    ryo_state.job_setup_step = new_job_setup;
    Ok(ApiResponse::Ok)
}
