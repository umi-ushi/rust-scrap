fn sample() -> Result<i64, String> {
    return Ok();
}

fn handle_match_simple() -> Result<i64, String> {
    // match式によるハンドリング
    let result = match sample() {
        Ok() => res,
        Err(err) => return Err(err),
    };
    Ok("ok")
}

fn handle_operator_sample() -> Result<i64, String> {
    let result = sample()?; // Resultのエラーがレスポンスされた場合は即座に早期リターンされる
    Ok()
}

#[derive(Error, Debug)]
pub enum ValidationError {
    #[error("{0}未満にしてください")]
    Less(String),
    #[error("{0}以下にしてください")]
    LessEq(String)
}

#[derive(Error, Debug)]
pub enum ApiError {
    #[error("バリデーションエラー: {0}")]
    Validation(#[from] ValidationError),
    #[error("データベースエラー: {0}")]
    Database(String),
    #[error("認証エラー")]
    Auth,
}

fn info_response(api_error: ApiError) -> (StatusCode, String) {
    match api_error {
        ApiError::Validation(e) => {
            (StatusCode::BadRequest, e.to_string())
        },
        ApiError::Auth => {
            (StatusCode::Unauthorized, "認証に失敗しました。".to_string())
        },
        _ => {
            (StatusCode::InternalServerError, "サーバーエラーが発生しました".to_string())
        }
    }
}

fn analyze_range(value: i32) {
    match value {
        // @によるバインディングパターンにてパターンマッチした値を束縛できる
        v @ 10..=100 => println!("{v}は10以上100以下の範囲内です"),
        neg @ ..=-1 => println!("{neg}は負の値です"),
        other => println!("{other}は範囲外です")
    }
}