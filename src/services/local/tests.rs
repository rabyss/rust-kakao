use super::api::*;
use super::model::*;
use crate::KakaoCred;

#[test]
fn test_search_address() -> Result<(), failure::Error> {
    let api_key = dotenv::var("KAKAO_REST_API_KEY")?;
    let client = KakaoLocal::new(&KakaoCred::new(&api_key));
    let res = client.search_address(&LocalSearchAddressInput {
        query: "전북 삼성동 100".to_string(),
        ..Default::default()
    })?;

    println!("{:#?}", res);

    Ok(())
}

#[test]
fn test_geo_coord_to_region_code() -> Result<(), failure::Error> {
    let api_key = dotenv::var("KAKAO_REST_API_KEY")?;
    let client = KakaoLocal::new(&KakaoCred::new(&api_key));
    let res = client.geo_coord_to_region_code(&LocalGeoCoordToRegionCodeInput {
        x: 127.1086228,
        y: 37.4012191,
        ..Default::default()
    })?;

    println!("{:#?}", res);

    Ok(())
}

#[test]
fn test_geo_coord_to_address() -> Result<(), failure::Error> {
    let api_key = dotenv::var("KAKAO_REST_API_KEY")?;
    let client = KakaoLocal::new(&KakaoCred::new(&api_key));
    let res = client.geo_coord_to_address(&LocalGeoCoordToAddressInput {
        x: 127.423084873712,
        y: 37.0789561558879,
        ..Default::default()
    })?;

    println!("{:#?}", res);

    Ok(())
}

#[test]
fn test_geo_trans_coord() -> Result<(), failure::Error> {
    let api_key = dotenv::var("KAKAO_REST_API_KEY")?;
    let client = KakaoLocal::new(&KakaoCred::new(&api_key));
    let res = client.geo_trans_coord(&LocalGeoTranscoordInput {
        x: 160710.37729270622,
        y: -4388.879299157299,
        input_coord: Some("WTM".to_string()),
        output_coord: Some("WGS84".to_string())
    })?;

    println!("{:#?}", res);

    Ok(())
}

#[test]
fn test_search_keyword() -> Result<(), failure::Error> {
    let api_key = dotenv::var("KAKAO_REST_API_KEY")?;
    let client = KakaoLocal::new(&KakaoCred::new(&api_key));
    let res = client.search_keyword(&LocalSearchKeywordInput {
        query: "카카오프렌즈".to_string(),
        x: Some(String::from("127.06283102249932")),
        y: Some(String::from("37.514322572335935")),
        radius: Some(20000),
        ..Default::default()
    })?;

    println!("{:#?}", res);

    Ok(())
}

#[test]
fn test_search_category() -> Result<(), failure::Error> {
    let api_key = dotenv::var("KAKAO_REST_API_KEY")?;
    let client = KakaoLocal::new(&KakaoCred::new(&api_key));
    let res = client.search_category(&LocalSearchCategoryInput {
        category_group_code: "PM9".to_string(),
        rect: Some(String::from("127.0561466,37.5058277,127.0602340,37.5142554")),
        ..Default::default()
    })?;

    println!("{:#?}", res);

    Ok(())
}