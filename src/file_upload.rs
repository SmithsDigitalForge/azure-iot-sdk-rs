use sha2::Digest;
use reqwest::header::{AUTHORIZATION, CONTENT_TYPE};
use serde::{Serialize, Deserialize};
use serde_json::{to_string, from_str};

const APPLICATION_JSON_TYPE : &str = "application/json";


#[derive(Serialize)]
struct FileUploadRequest {
    blobName: String,
}

#[derive(Deserialize, Debug)]
struct FileUploadResponse {
    correlationId: String,
    hostName: String,
    containerName: String,
    blobName: String,
    sasToken: String,
}

fn file_upload_request_url(hub: &str, device_id: &str) -> String {
    format!("https://{}/devices/{}/files?api-version=2018-06-30", hub, device_id)
}

// Following: https://docs.microsoft.com/en-us/azure/iot-hub/iot-hub-devguide-file-upload
pub(crate) async fn send_file_to_iot_hub(filename: &str, hub: &str, device_id: &str, sas: &str) -> Result<(), Box<dyn std::error::Error>> {
    println!("send_file called");
    let url = file_upload_request_url(hub, device_id);
    let blob_request = FileUploadRequest { blobName: filename.to_string() };
    let client = reqwest::Client::new();
    println!("url {}", url);
    println!("sas token {}", sas);
    let res = client
        .post(&url)
        .header(AUTHORIZATION, sas)
        .header(CONTENT_TYPE, APPLICATION_JSON_TYPE)
        .body(serde_json::to_string(&blob_request)?)
        .send()
        .await?;
    let reply_text = res.text().await?;
    println!("Reply text {}", reply_text);
    let reply: FileUploadResponse = serde_json::from_str(&reply_text)?;
    println!("Reply {:#?}", reply);
    Ok(())
}