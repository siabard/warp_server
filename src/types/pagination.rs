use std::collections::HashMap;

use handle_errors::Error;

/// Pagination 구조체는 쿼리 매개변수로부터
/// 추출된다.
#[derive(Default, Debug)]
pub struct Pagination {
    /// 반환될 마지막 아이템의 인덱스
    pub limit: Option<u32>,
    /// 반환될 첫번째 아이템의 인덱스
    pub offset: u32,
}

/// 파라미터를 `/questions` 경로로부터 추출하기
/// # 예제 쿼리
/// 이 경로에 대한 GET 요청에는 반환받기 원하는 질문만 반환받도록
/// 페이지 정보가 추가될 수 있다.
/// `/questions?start=1&end=10`
/// ## 사용예
/// ```rust
/// let mut query = HashMap::new();
/// query.insert("limit".to_string(), "1".to_string());
/// query.insert("offset".to_string(), "10".to_string());
/// let p = types::pagination::extract_pagination(query).unwrap();
/// assert_eq!(p.limit, 1);
/// assert_eq!(p.offset, 10);
/// ```
pub fn extract_pagination(params: HashMap<String, String>) -> Result<Pagination, Error> {
    // 나중에 더 개선가능하다.
    if params.contains_key("start") && params.contains_key("end") {
        return Ok(Pagination {
            // "limit" 매개변수를 쿼리에서 가져와
            // 숫자로 변환을 시도한다.
            limit: Some(
                params
                    .get("limit")
                    .unwrap()
                    .parse::<u32>()
                    .map_err(Error::ParseError)?,
            ),
            // "end" 매개변수를 쿼리에서 가져와
            // 숫자로 변환을 시도한다.
            offset: params
                .get("offset")
                .unwrap()
                .parse::<u32>()
                .map_err(Error::ParseError)?,
        });
    }

    Err(Error::MissingParameters)
}
