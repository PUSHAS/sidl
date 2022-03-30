#[derive(Debug, Serialize, Deserialize)]
pub struct ShopifyProductsResponse {
	pub products: Vec<ShopifyProduct>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct ShopifyProduct {
	pub id: i64,
	pub title: String,
	pub body_html: String,
	pub vendor: String,
	pub product_type: String,
	pub created_at: String,
	pub handle: String,
	pub updated_at: String,
	pub published_at: Option<serde_json::Value>,
	pub template_suffix: Option<String>,
	pub status: String,
	pub published_scope: String,
	pub tags: String,
	pub admin_graphql_api_id: String,
	pub variants: Vec<Variant>,
	pub options: Vec<ShopifyOption>,
	pub images: Vec<Image>,
	pub image: Option<Image>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct Image {
	pub id: i64,
	pub product_id: i64,
	pub position: i64,
	pub created_at: String,
	pub updated_at: String,
	pub alt: Option<serde_json::Value>,
	pub width: i64,
	pub height: i64,
	pub src: String,
	pub variant_ids: Vec<Option<serde_json::Value>>,
	pub admin_graphql_api_id: String,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct ShopifyOption {
	pub id: i64,
	pub product_id: i64,
	pub name: String,
	pub position: i64,
	pub values: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct Variant {
	pub id: i64,
	pub product_id: i64,
	pub title: String,
	pub price: String,
	pub sku: Option<String>,
	pub position: i64,
	pub inventory_policy: String,
	pub compare_at_price: Option<serde_json::Value>,
	pub fulfillment_service: String,
	pub inventory_management: String,
	pub option1: Option<String>,
	pub option2: Option<String>,
	pub option3: Option<String>,
	pub created_at: String,
	pub updated_at: String,
	pub taxable: bool,
	pub barcode: Option<String>,
	pub grams: i64,
	pub image_id: Option<serde_json::Value>,
	pub weight: f64,
	pub weight_unit: String,
	pub inventory_item_id: i64,
	pub inventory_quantity: i64,
	pub old_inventory_quantity: i64,
	pub requires_shipping: bool,
	pub admin_graphql_api_id: String,
}
