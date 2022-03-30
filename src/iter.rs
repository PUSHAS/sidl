use crate::types::product::{ShopifyProduct, ShopifyProductsResponse};
use crate::SHOPIFY_ACCESS_TOKEN;
use anyhow::Result;
use async_trait::async_trait;

pub struct ShopifyProducts {
	client: reqwest::Client,
	products: <Vec<ShopifyProduct> as IntoIterator>::IntoIter,
	link: String,
	done: bool,
}

#[async_trait::async_trait]
pub trait AsyncIterator {
	type Item;

	async fn next(&mut self) -> Option<Self::Item>;
}

impl ShopifyProducts {
	pub fn new(link: String) -> Result<Self> {
		let mut def_headers = reqwest::header::HeaderMap::new();
		def_headers.append("X-Shopify-Access-Token", SHOPIFY_ACCESS_TOKEN.parse()?);
		let client = reqwest::Client::builder()
			.default_headers(def_headers)
			.build()?;

		Ok(ShopifyProducts {
			client,
			products: vec![].into_iter(),
			link,
			done: false,
		})
	}

	async fn try_next(&mut self) -> Result<Option<ShopifyProduct>> {
		if let Some(product) = self.products.next() {
			return Ok(Some(product));
		}
		if self.done {
			return Ok(None);
		}

		let response = self.client.get(&self.link).send().await?;
		let headers = response.headers().clone();

		let data: ShopifyProductsResponse = response.json().await?;
		self.products = data.products.into_iter();

		let parsed_link = parse_link_header::parse(headers.get("link").unwrap().to_str().unwrap())?;
		let link = parsed_link.get(&Some("next".to_string()));

		match link {
			Some(link) => {
				self.link = link.raw_uri.to_string();
			}
			None => {
				self.done = true;
			}
		}

		Ok(self.products.next())
	}
}

#[async_trait]
impl AsyncIterator for ShopifyProducts {
	type Item = Result<ShopifyProduct>;

	async fn next(&mut self) -> Option<Self::Item> {
		match self.try_next().await {
			Ok(Some(product)) => Some(Ok(product)),
			Ok(None) => None,
			Err(err) => Some(Err(err)),
		}
	}
}
