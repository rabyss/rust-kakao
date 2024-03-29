use crate::KakaoCred;
use super::model::*;
use crate::services::KakaoService;

pub struct KakaoLocal {
    service: Box<KakaoService>
}

impl KakaoLocal {
    pub fn new (cred: &KakaoCred) -> Self {
        KakaoLocal {
            service: Box::new(KakaoService::new(cred))
        }
    }

    pub fn search_address(&self, input: &LocalSearchAddressInput) -> Result<LocalSearchAddressOutput, failure::Error> {
        let api = "v2/local/search/address.json";
        self.service.get(api, input)
    }

    pub fn geo_coord_to_region_code(&self, input: &LocalGeoCoordToRegionCodeInput) -> Result<LocalGeoCoordToRegionCodeOutput, failure::Error> {
        let api = "v2/local/geo/coord2regioncode";
        self.service.get(api, input)
    }

    pub fn geo_coord_to_address(&self, input: &LocalGeoCoordToAddressInput) -> Result<LocalGeoCoordToAddressOutput, failure::Error> {
        let api = "v2/local/geo/coord2address.json";
        self.service.get(api, input)
    }

    pub fn geo_trans_coord(&self, input: &LocalGeoTranscoordInput) -> Result<LocalGeoTranscoordOutput, failure::Error> {
        let api = "v2/local/geo/transcoord.json";
        self.service.get(api, input)
    }

    pub fn search_keyword(&self, input: &LocalSearchKeywordInput) -> Result<LocalSearchKeywordOutput, failure::Error> {
        let api = "v2/local/search/keyword.json";
        self.service.get(api, input)
    }

    pub fn search_category(&self, input: &LocalSearchCategoryInput) -> Result<LocalSearchCategoryOutput, failure::Error> {
        let api = "v2/local/search/category.json";
        self.service.get(api, input)
    }
}

