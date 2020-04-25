extern crate json;
extern crate ureq;

#[derive(Clone, Copy)]
enum MetadataUrls {
    InstanceId,
    AmiId,
    AccountId,
    AvailabilityZone,
}

impl Into<&'static str> for MetadataUrls {
    fn into(self) -> &'static str {
        match self {
            MetadataUrls::InstanceId => "http://169.254.169.254/latest/meta-data/instance-id",
            MetadataUrls::AmiId => "http://169.254.169.254/latest/meta-data/ami-id",
            MetadataUrls::AccountId => {
                "http://169.254.169.254/latest/meta-data/identity-credentials/ec2/info"
            }
            MetadataUrls::AvailabilityZone => {
                "http://169.254.169.254/latest/meta-data/placement/availability-zone"
            }
        }
    }
}

fn identity_credentials_to_account_id(ident_creds: &str) -> String {
    let parsed = json::parse(ident_creds).unwrap();
    parsed["AccountId"].to_string()
}

fn availability_zone_to_region(availability_zone: &str) -> Option<&'static str> {
    const REGIONS: &[&str] = &[
        "ap-south-1",
        "eu-west-3",
        "eu-north-1",
        "eu-west-2",
        "eu-west-1",
        "ap-northeast-3",
        "ap-northeast-2",
        "ap-northeast-1",
        "sa-east-1",
        "ca-central-1",
        "ap-southeast-1",
        "ap-southeast-2",
        "eu-central-1",
        "us-east-1",
        "us-east-2",
        "us-west-1",
        "us-west-2",
        "cn-north-1",
        "cn-northwest-1",
    ];

    for region in REGIONS {
        if availability_zone.starts_with(region) {
            return Some(region);
        }
    }

    None
}

#[derive(Debug, Default)]
pub struct InstanceMetadataClient;

impl InstanceMetadataClient {
    pub fn new() -> Self {
        Self {}
    }

    fn get_token(&self) -> Option<String> {
        const TOKEN_API_URL: &str = "http://169.254.169.254/latest/api/token";

        let resp = ureq::put(TOKEN_API_URL)
            .set("X-aws-ec2-metadata-token-ttl-seconds", "21600")
            .call();

        resp.into_string().ok()
    }

    pub fn get(&self) -> Option<InstanceMetadata> {
        let token = self.get_token()?;
        let instance_id = ureq::get(MetadataUrls::InstanceId.into())
            .set("X-aws-ec2-metadata-token", &token)
            .call()
            .into_string()
            .ok()?;

        let ident_creds = ureq::get(MetadataUrls::AccountId.into())
            .set("X-aws-ec2-metadata-token", &token)
            .call()
            .into_string()
            .ok()?;
        let account_id = identity_credentials_to_account_id(&ident_creds);

        let ami_id = ureq::get(MetadataUrls::AmiId.into())
            .set("X-aws-ec2-metadata-token", &token)
            .call()
            .into_string()
            .ok()?;

        let availability_zone = ureq::get(MetadataUrls::AvailabilityZone.into())
            .set("X-aws-ec2-metadata-token", &token)
            .call()
            .into_string()
            .ok()?;
        let region = availability_zone_to_region(&availability_zone)?;

        let metadata = InstanceMetadata {
            _unused: (),
            region,
            availability_zone,
            instance_id,
            account_id,
            ami_id,
        };

        Some(metadata)
    }
}

#[derive(Debug, Clone)]
pub struct InstanceMetadata {
    _unused: (),
    region: &'static str,
    availability_zone: String,
    instance_id: String,
    account_id: String,
    ami_id: String,
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
