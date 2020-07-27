# EC2 Instance Metadata Crate
This crate provides functionality for getting the Instance Metadata from
an EC2 instance. This API makes use of the v2 API to remain future facing.

# Installing/Using
Add the following line to your `Cargo.toml`:

```
ec2_instance_metadata = "0.2"
```

# Example Code:

```
extern crate ec2_instance_metadata;
let client = ec2_instance_metadata::InstanceMetadataClient::new();
let metadata = client.get().unwrap();
```
