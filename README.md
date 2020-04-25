# AWS Instance Metadata Crate
This crate provides functionality for getting the Instance Metadata from
an EC2 instance. This API makes use of the v2 API to remain future facing.

# Installing/Using
Add the following line to your `Cargo.toml`:

```
aws_instance_metadata = "0.1"
```

# Example Code:

```
extern crate aws_instance_metadata;
let client = aws_instance_metadata::InstanceMetadataClient::new();
let metadata = client.get().unwrap();
```
