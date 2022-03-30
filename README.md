# SIDL 
SIDL (Shopify Image Downloader) is a program to programmatically fetch all your Shopify product images and back them up to Amazon S3.

## Running SIDL
Building and running SIDL is fairly simple. 

1. Clone the source code  

`git clone https://github.com/PUSHAS/sidl`

2. Rename `.env.example` and enter your environment variables.  

`mv .env.example .env`

3. Run the program  

`cargo run`

## Data Format
SIDL stores your product images in the following format:
```bash
.
└── product_id # shopify product id
   ├── 1.jpg # first image
   ├── 2.jpg # second image
   ├── 3.jpg # third image
   ├── 4.jpg # nth image
   └── data.toml # product data
```
The folder housing the images is titled after the Shopify product id. Images are stored by their position on the product carousel. The `data.toml` file stores the Shopify product id and title.