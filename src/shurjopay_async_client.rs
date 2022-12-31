pub async fn get_async(url: String) -> Result<String, Box<dyn std::error::Error>> 
{
    let resp = reqwest::get(url)
        .await?;
    // println!("status code:{:#?}", resp.status().clone());
    let response =  resp.text().await?;
    // println!("response string: {:#?}", response);
    Ok(response)
}