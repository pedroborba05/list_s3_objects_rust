extern crate rusoto_core;
extern crate rusoto_s3;

use rusoto_core::Region;
use rusoto_s3::{ListObjectsRequest, S3, S3Client};

fn extract_id_from_key(key: &str) -> Option<String> {
    let parts: Vec<&str> = key.split('-').collect();
    if parts.len() >= 2 {
        let id_with_extension = parts[1];
        if let Some(index) = id_with_extension.find('.') {
            return Some(id_with_extension[..index].to_string());
        }
    }
    None
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let region = Region::SaEast1;
    let client = S3Client::new(region);

    let bucket_name = "app-dev-catracaimagens";

    let prefix = "testing"; 

    let list_objects_request = ListObjectsRequest {
        bucket: bucket_name.to_string(),
        prefix: Some(prefix.to_string()),
        ..Default::default()
    };

    let result = client.list_objects(list_objects_request).await?;

    let mut ids: Vec<String> = Vec::new();

    if let Some(objects) = result.contents {
        println!("IDs dos objetos com prefixo '{}' no bucket '{}':", prefix, bucket_name);
        for object in objects {
            if let Some(key) = object.key {
                if let Some(id) = extract_id_from_key(&key) {
                    ids.push(id.clone()); 
                    println!("ID: {}", id);
                }
            }
        }
    } else {
        println!("Nenhum objeto com prefixo '{}' encontrado no bucket '{}'", prefix, bucket_name);
    }

    println!("IDs armazenados no vetor: {:?}", ids);

    Ok(())
}
