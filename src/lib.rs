use chrono::TimeZone;

pub mod utils;

#[derive(Clone, Debug)]
pub struct Shopify {
    shared_secret: Option<String>,
    api_key: String,
    query_url: String,
    rest_url: String,
    shop: String,
}

#[derive(Clone, Debug)]
pub enum ShopifyAPIVersion {
    /// Deprecated
    V2021_10,

    /// Will be deprecated soon
    V2022_01,

    V2022_04,
    V2022_07,

    /// Latest stable version
    V2022_10,

    /// Release candidate
    V2023_01,
    Unstable,
}

/// Get the end of support date for a given API version
/// # Example
///
/// ```
/// use chrono::TimeZone;
/// use shopify_api::{ get_end_of_support_date, ShopifyAPIVersion };
///
/// assert_eq!(
///     get_end_of_support_date(&ShopifyAPIVersion::V2023_01),
///     chrono::Utc.ymd(2023, 1, 31).and_hms(23, 59, 59)
/// );
/// ```
pub fn get_end_of_support_date(api_version: &ShopifyAPIVersion) -> chrono::DateTime<chrono::Utc> {
    match api_version {
        ShopifyAPIVersion::V2021_10 => chrono::Utc.ymd(2021, 10, 31).and_hms(23, 59, 59),
        ShopifyAPIVersion::V2022_01 => chrono::Utc.ymd(2022, 1, 31).and_hms(23, 59, 59),
        ShopifyAPIVersion::V2022_04 => chrono::Utc.ymd(2022, 4, 30).and_hms(23, 59, 59),
        ShopifyAPIVersion::V2022_07 => chrono::Utc.ymd(2022, 7, 31).and_hms(23, 59, 59),
        ShopifyAPIVersion::V2022_10 => chrono::Utc.ymd(2022, 10, 31).and_hms(23, 59, 59),
        ShopifyAPIVersion::V2023_01 => chrono::Utc.ymd(2023, 1, 31).and_hms(23, 59, 59),
        ShopifyAPIVersion::Unstable => chrono::Utc.ymd(9999, 12, 31).and_hms(23, 59, 59),
    }
}

/// Check if a given API version is deprecated because it is not supported anymore
/// # Example
/// ```
/// use shopify_api::{ is_deprecated, ShopifyAPIVersion };
/// assert_eq!(is_deprecated(&ShopifyAPIVersion::V2021_10), true);
/// assert_eq!(is_deprecated(&ShopifyAPIVersion::V2023_01), false);
/// ```
pub fn is_deprecated(api_version: &ShopifyAPIVersion) -> bool {
    let max_date = get_end_of_support_date(api_version);

    chrono::Utc::now() > max_date
}

/// Transform the enum type of the API version to a string
/// # Example
/// ```
/// use shopify_api::{ api_version_to_string, ShopifyAPIVersion };
/// assert_eq!(api_version_to_string(&ShopifyAPIVersion::V2021_10), "2021-10");
/// ```
pub fn api_version_to_string(api_version: &ShopifyAPIVersion) -> String {
    match api_version {
        ShopifyAPIVersion::V2021_10 => "2021-10".to_string(),
        ShopifyAPIVersion::V2022_01 => "2022-01".to_string(),
        ShopifyAPIVersion::V2022_04 => "2022-04".to_string(),
        ShopifyAPIVersion::V2022_07 => "2022-07".to_string(),
        ShopifyAPIVersion::V2022_10 => "2022-10".to_string(),
        ShopifyAPIVersion::V2023_01 => "2023-01".to_string(),
        ShopifyAPIVersion::Unstable => "unstable".to_string(),
    }
}

impl Shopify {
    /// Create a new Shopify client
    /// # Example
    /// ```
    /// use shopify_api::Shopify;
    /// let shopify = Shopify::new("myshop", "myapikey", Some("mysharedsecret"));
    /// // or without shared secret
    /// let shopify = Shopify::new("myshop", "myapikey", None);
    /// ```
    pub fn new(shop: &str, api_key: &str, shared_secret: Option<&str>) -> Shopify {
        let query_url = format!("https://{}/admin/api/2020-04/graphql.json", shop);
        let rest_url = format!("https://{}/admin/api/2020-04/", shop);

        Shopify {
            shared_secret: shared_secret.map(|secret| secret.to_string()),
            api_key: api_key.to_string(),
            query_url,
            rest_url,
            shop: shop.to_string(),
        }
    }

    /// Get the shop name
    /// # Example
    /// ```
    /// use shopify_api::Shopify;
    /// let shopify = Shopify::new("my-shop", "my-api-key", Some("my-shared-secret"));
    /// assert_eq!(shopify.get_shop(), "my-shop");
    /// ```
    pub fn get_shop(&self) -> &str {
        self.shop.as_ref()
    }

    /// Set the API Key
    /// # Example
    /// ```
    /// use shopify_api::Shopify;
    /// let mut shopify = Shopify::new("myshop", "myapikey", Some("mysharedsecret"));
    /// shopify.set_api_key("newapikey");
    /// ```
    /// # Errors
    /// This function returns an error if the API key is empty
    pub fn set_api_key(&mut self, api_key: &str) -> Result<&mut Shopify, String> {
        if api_key.is_empty() {
            return Err("API key cannot be empty".to_string());
        }

        self.api_key = api_key.to_string();
        Ok(self)
    }
}