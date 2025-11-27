// async fn db_result_match(result: sea_orm::QueryResult) -> Result<(), Box<dyn std::error::Error>> {
//     // 비동기 코드

//     match result  {
//         sea_orm::QueryResult::Ok(rows) => {

//             println!("쿼리 성공, 행 수: {}", rows.len());
//             Ok(("쿼리 성공".into()))
//         }
//         _ => Err("쿼리 실패".into()),
//     }

// }
