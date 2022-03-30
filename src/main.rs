#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate log;
#[macro_use]
extern crate async_trait;
extern crate serde;
extern crate serde_json;

pub mod iter;
pub mod types;
pub mod util;

use anyhow::Result;
use env_logger::Env;
use iter::{AsyncIterator, ShopifyProducts};
use lazy_static::lazy_static;
use std::env::var;
use util::link_ext;

lazy_static! {
	pub static ref SHOPIFY_ACCESS_TOKEN: String = var("SHOPIFY_ACCESS_TOKEN").unwrap();
	static ref AWS_BUCKET: String = var("AWS_BUCKET").unwrap();
	static ref SHOPIFY_STORE: String = var("SHOPIFY_STORE").unwrap();
}

#[derive(Serialize)]
struct TOMLData {
	shopify_id: i64,
	title: String,
}

#[tokio::main]
async fn main() -> Result<()> {
	env_logger::Builder::from_env(Env::default().default_filter_or("info")).init();

	let aws_config = aws_config::load_from_env().await;
	let s3_client = aws_sdk_s3::Client::new(&aws_config);

	let start = format!(
		"https://{}/admin/api/2021-07/products.json?limit=100",
		SHOPIFY_STORE.as_str()
	);
	let mut iter = ShopifyProducts::new(start.to_string())?;

	while let Some(product) = iter.next().await {
		let product = product?;
		info!("Working on product: {}", product.title);

		let data = TOMLData {
			title: product.title,
			shopify_id: product.id,
		};
		let toml = toml::to_string(&data)?;

		// for each product.image, download the image and upload to s3 with the folder name as the product id
		for image in &product.images {
			let ext = link_ext(image.src.split('?').next().unwrap_or("foo.jpg")).unwrap_or("jpg");
			let path = format!("{}/{}.{}", product.id, image.position, ext);

			// check if the file already exists
			let exists = s3_client
				.head_object()
				.bucket(AWS_BUCKET.as_str())
				.key(&path)
				.send()
				.await;

			if exists.is_ok() {
				info!("File already exists: {}", path);
				continue;
			};

			let response = reqwest::get(&image.src).await?;

			let request = s3_client
				.put_object()
				.bucket(AWS_BUCKET.as_str())
				.key(&path)
				.body(response.bytes().await?.into());

			request.send().await?;
			info!("Uploaded Image #{} for {}", image.position, &data.title);
		}

		// upload the toml file
		let request = s3_client
			.put_object()
			.bucket(AWS_BUCKET.as_str())
			.key(format!("{}/data.toml", product.id))
			.body(toml.into_bytes().into());
		let _ = request.send().await?;
		info!("Uploaded toml for {}\n", &data.title);
	}

	Ok(())
}
