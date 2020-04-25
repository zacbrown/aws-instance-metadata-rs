extern crate aws_instance_metadata;

fn main() {
    let client = aws_instance_metadata::InstanceMetadataClient::new();
    let metadata = client.get();
    println!("metadata:\n{:?}", metadata.unwrap());
}
