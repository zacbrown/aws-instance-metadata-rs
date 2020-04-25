extern crate ec2_instance_metadata;

fn main() {
    let client = ec2_instance_metadata::InstanceMetadataClient::new();
    let metadata = client.get();
    println!("metadata:\n{:?}", metadata.unwrap());
}
