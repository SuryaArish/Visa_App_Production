use axum::{
    middleware::Next,
    extract::Request,
    response::{Response, IntoResponse},
    http::StatusCode,
};

pub async fn auth_middleware(request: Request, next: Next) -> impl IntoResponse {
    let headers = request.headers();
    
    let token = match headers.get("authorization") {
        Some(auth_header) => {
            match auth_header.to_str() {
                Ok(auth_str) => {
                    if auth_str.starts_with("Bearer ") {
                        &auth_str[7..]
                    } else {
                        return (StatusCode::UNAUTHORIZED, "Unauthorized data").into_response();
                    }
                }
                Err(_) => return (StatusCode::UNAUTHORIZED, "Unauthorized data").into_response(),
            }
        }
        None => return (StatusCode::UNAUTHORIZED, "Unauthorized data").into_response(),
    };
    
    if verify_supabase_token(token).await {
        next.run(request).await
    } else {
        (StatusCode::UNAUTHORIZED, "Unauthorized data").into_response()
    }
}

async fn verify_supabase_token(token: &str) -> bool {
    let supabase_url = match std::env::var("SUPABASE_URL") {
        Ok(url) => url,
        Err(_) => return false,
    };
    
    let supabase_api_key = match std::env::var("SUPABASE_API_KEY") {
        Ok(key) => key,
        Err(_) => return false,
    };
    
    let client = reqwest::Client::new();
    let url = format!("{}/auth/v1/user", supabase_url);
    
    match client
        .get(&url)
        .header("Authorization", format!("Bearer {}", token))
        .header("apikey", supabase_api_key)
        .send()
        .await
    {
        Ok(response) => response.status().is_success(),
        Err(_) => false,
    }
}