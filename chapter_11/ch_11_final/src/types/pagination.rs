use handle_errors::Error;
use std::collections::HashMap;

/// Pagination 구조체는 쿼리 매개변수에서
/// 추출된다
#[derive(Default, Debug, PartialEq)]
pub struct Pagination {
    /// 반환될 마지막 아이템의 인덱스
    pub limit: Option<u32>,
    /// 반환될 첫 번째 아이템의 인덱스
    pub offset: u32,
}

/// 매개변수를 /questions 경로에서 추출하기
/// # 예제 쿼리
/// 이 경로에 대한 GET 요청에는 반환 받기 원하는 질문만 반환 받도록
/// 페이지 정보가 추가될 수 있다
/// /questions?start=1&end=10
/// # 사용 예
/// ```rust
/// let mut query = HashMap::new();
/// query.insert("limit".to_string(), "1".to_string());
/// query.insert("offset".to_string(), "10".to_string());
/// let p = types::pagination::extract_pagination(query).unwrap();
/// assert_eq!(p.limit, 1);
/// assert_eq!(p.offset, 10);
/// ```
pub fn extract_pagination(params: HashMap<String, String>) -> Result<Pagination, Error> {
    // 나중에 더 개선할 수 있다
    if params.contains_key("limit") && params.contains_key("offset") {
        return Ok(Pagination {
            // limit 매개변수를 쿼리에서 가져와
            // 숫자로 변환을 시도한다
            limit: Some(
                params
                    .get("limit")
                    .unwrap()
                    .parse::<u32>()
                    .map_err(Error::ParseError)?,
            ),
            // offset 매개변수를 쿼리에서 가져와
            // 숫자로 변환하려고 한다
            offset: params
                .get("offset")
                .unwrap()
                .parse::<u32>()
                .map_err(Error::ParseError)?,
        });
    }

    Err(Error::MissingParameters)
}

#[cfg(test)]
mod pagination_tests {
    use super::{extract_pagination, Error, HashMap, Pagination};

    #[test]
    fn valid_pagination() {
        let mut params = HashMap::new();
        params.insert(String::from("limit"), String::from("1"));
        params.insert(String::from("offset"), String::from("1"));
        let pagination_result = extract_pagination(params);
        let expected = Pagination {
            limit: Some(1),
            offset: 1,
        };
        assert_eq!(pagination_result.unwrap(), expected);
    }

    #[test]
    fn missing_offset_parameter() {
        let mut params = HashMap::new();
        params.insert(String::from("limit"), String::from("1"));

        let pagination_result = format!("{}", extract_pagination(params).unwrap_err());
        let expected = format!("{}", Error::MissingParameters);

        assert_eq!(pagination_result, expected);
    }
}
