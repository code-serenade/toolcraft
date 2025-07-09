use toolcraft::s3::generate_post_policy;

fn main() {
    let response = generate_post_policy(
        "ROOTNAME",
        "CHANGEME123",
        "test-bucket",
        "upload/",
        "local_region",
        "http://127.0.0.1:9000",
        10,
    );
    println!("{:#?}", response);
}
