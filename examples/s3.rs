use toolcraft::s3::generate_post_policy;

fn main() {
    let response = generate_post_policy(
        "ROOTNAME",
        "secret_key",
        "bucket_name",
        "key_prefix/",
        "local_region",
        "http://127.0.0.1:9000",
        10,
    );
    println!("{:#?}", response);
}
