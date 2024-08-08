# Axum Cloudflare R2

This repository is an example of a multipart upload to Cloudflare [R2](https://www.cloudflare.com/developer-platform/r2/) using AWS S3 [Rust SDK](https://github.com/awslabs/aws-sdk-rust).

## Getting started

Set environment variables.

```
R2_ACCOUNT_ID=
R2_ACCESS_ID=
R2_ACCESS_SECRET=
R2_BUCKET_NAME=
R2_REGION=
```

Launch HTTP server with your environment variables loaded. For example using [dotenvx](https://dotenvx.com/).

```sh
$ dotenvx run -- cargo run
```
