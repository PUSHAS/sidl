#[derive(Debug, Serialize, Deserialize)]
pub struct ShopifyProductsResponse {
	pub products: Vec<ShopifyProduct>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct ShopifyProduct {
	pub id: i64,
	pub title: String,
	pub images: Vec<Image>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct Image {
	pub position: i64,
	pub src: String,
}