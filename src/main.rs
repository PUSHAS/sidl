#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate log;
extern crate serde;

pub mod iter;
pub mod types;
pub mod util;

use crate::iter::{AsyncIterator, ShopifyProducts};
use crate::util::link_ext;
use anyhow::Result;
use aws_sdk_s3::output::PutObjectOutput;
use env_logger::Env;
use lazy_static::lazy_static;
use std::env::var;
use tokio::task::JoinHandle;

lazy_static! {
	pub static ref SHOPIFY_ACCESS_TOKEN: String = var("SHOPIFY_ACCESS_TOKEN").unwrap();
	static ref AWS_BUCKET: String = var("AWS_BUCKET").unwrap();
	static ref SHOPIFY_STORE: String = var("SHOPIFY_STORE").unwrap();
}

#[derive(Serialize, Clone)]
struct TOMLData {
	shopify_id: i64,
	title: String,
}

async fn handle_product(
	s3_client: &aws_sdk_s3::Client,
	product: types::product::ShopifyProduct,
) -> Result<(), anyhow::Error> {
	info!("Working on product: {}", product.title);

	let data = TOMLData {
		title: product.title,
		shopify_id: product.id,
	};
	let toml = toml::to_string(&data)?;

	let handles: Vec<JoinHandle<Result<Option<PutObjectOutput>, anyhow::Error>>> = product
		.images
		.into_iter()
		.map(|image| -> JoinHandle<Result<Option<PutObjectOutput>, _>> {
			let s3_client = s3_client.clone();
			let data = data.clone();
			tokio::spawn(async move {
				let ext =
					link_ext(image.src.split('?').next().unwrap_or("foo.jpg")).unwrap_or("jpg");
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
					return Ok(None);
				};

				let response = reqwest::get(&image.src).await?;

				let request = s3_client
					.put_object()
					.bucket(AWS_BUCKET.as_str())
					.key(&path)
					.body(response.bytes().await?.into());

				let res = request.send().await?;
				info!("Uploaded Image #{} for {}", image.position, &data.title);

				Ok(Some(res))
			})
		})
		.collect();

	for handle in handles {
		let _ = handle.await?;
	}

	let request = s3_client
		.put_object()
		.bucket(AWS_BUCKET.as_str())
		.key(format!("{}/data.toml", product.id))
		.body(toml.into_bytes().into());

	let _ = request.send().await?;
	info!("Uploaded toml for {}\n", &data.title);

	Ok(())
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
	let mut products = ShopifyProducts::new(start.to_string())?;

	while let Some(product) = products.next().await {
		handle_product(&s3_client, product?).await?;
	}

	Ok(())
}
