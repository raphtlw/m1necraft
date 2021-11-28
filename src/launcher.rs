use std::error::Error;
use std::ops::Deref;
use std::path::PathBuf;
use std::{env, fs, process};

use reqwest::IntoUrl;
use serde_json::Value;

pub async fn download_file<T>(url: T, custom_file_name: Option<&str>) -> Result<(), Box<dyn Error>>
where
    T: IntoUrl + Clone + Copy,
{
    log::info!("Downloading from URL: {}", url.as_str());
    
    let file_blob = reqwest::get(url).await?.bytes().await?;
    let file_name = if let Some(value) = custom_file_name {
        value
    } else {
        url.as_str().split("/").last().unwrap()
    };
    
    let mut file_out_path = env::current_dir()?;
    file_out_path.push(file_name);
    
    log::info!("Writing file at path: {:#?}", file_out_path);

    fs::write(file_out_path, file_blob)?;

    Ok(())
}

/// Downloads the libraries needed for Minecraft to run and downloads
/// Minecraft itself.
/// 
/// During execution, it will move into the folder that contains the
/// minecraft "libraries" folder.
pub async fn download_mc_libraries() -> Result<(), Box<dyn Error>> {
    let cwd = env::current_dir()?;
    env::set_current_dir({
        let mut p = crate::MC_LIBS_PATH.get().unwrap().clone();
        p.push("libraries");
        p
    })?;
    
    if let Err(_) = futures::try_join!(
            // Specify library urls here
            
            download_file("https://launcher.mojang.com/v1/objects/1952d94a0784e7abda230aae6a1e8fc0522dba99/client.jar", Some("minecraft-1.16.4-client.jar")),
            download_file("https://libraries.minecraft.net/com/mojang/patchy/1.1/patchy-1.1.jar", None),
            download_file("https://libraries.minecraft.net/oshi-project/oshi-core/1.1/oshi-core-1.1.jar", None),
            download_file("https://libraries.minecraft.net/net/java/dev/jna/jna/4.4.0/jna-4.4.0.jar", None),
            download_file("https://libraries.minecraft.net/net/java/dev/jna/platform/3.4.0/platform-3.4.0.jar", None),
            download_file("https://libraries.minecraft.net/com/ibm/icu/icu4j/66.1/icu4j-66.1.jar", None),
            download_file("https://libraries.minecraft.net/com/mojang/javabridge/1.0.22/javabridge-1.0.22.jar", None),
            download_file("https://libraries.minecraft.net/net/sf/jopt-simple/jopt-simple/5.0.3/jopt-simple-5.0.3.jar", None),
            download_file("https://libraries.minecraft.net/io/netty/netty-all/4.1.25.Final/netty-all-4.1.25.Final.jar", None),
            download_file("https://libraries.minecraft.net/com/google/guava/guava/21.0/guava-21.0.jar", None),
            download_file("https://libraries.minecraft.net/org/apache/commons/commons-lang3/3.5/commons-lang3-3.5.jar", None),
            download_file("https://libraries.minecraft.net/commons-io/commons-io/2.5/commons-io-2.5.jar", None),
            download_file("https://libraries.minecraft.net/commons-codec/commons-codec/1.10/commons-codec-1.10.jar", None),
            download_file("https://libraries.minecraft.net/com/mojang/brigadier/1.0.17/brigadier-1.0.17.jar", None),
            download_file("https://libraries.minecraft.net/com/mojang/datafixerupper/4.0.26/datafixerupper-4.0.26.jar", None),
            download_file("https://libraries.minecraft.net/com/google/code/gson/gson/2.8.0/gson-2.8.0.jar", None),
            download_file("https://libraries.minecraft.net/com/mojang/authlib/2.0.27/authlib-2.0.27.jar", None),
            download_file("https://libraries.minecraft.net/org/apache/commons/commons-compress/1.8.1/commons-compress-1.8.1.jar", None),
            download_file("https://libraries.minecraft.net/org/apache/httpcomponents/httpclient/4.3.3/httpclient-4.3.3.jar", None),
            download_file("https://libraries.minecraft.net/commons-logging/commons-logging/1.1.3/commons-logging-1.1.3.jar", None),
            download_file("https://libraries.minecraft.net/org/apache/httpcomponents/httpcore/4.3.2/httpcore-4.3.2.jar", None),
            download_file("https://libraries.minecraft.net/it/unimi/dsi/fastutil/8.2.1/fastutil-8.2.1.jar", None),
            download_file("https://libraries.minecraft.net/org/apache/logging/log4j/log4j-api/2.8.1/log4j-api-2.8.1.jar", None),
            download_file("https://libraries.minecraft.net/org/apache/logging/log4j/log4j-core/2.8.1/log4j-core-2.8.1.jar", None),
            download_file("https://libraries.minecraft.net/com/mojang/text2speech/1.11.3/text2speech-1.11.3.jar", None),
            download_file("https://libraries.minecraft.net/com/mojang/text2speech/1.11.3/text2speech-1.11.3.jar", None),
            download_file("https://libraries.minecraft.net/com/mojang/text2speech/1.11.3/text2speech-1.11.3-natives-linux.jar", None),
            download_file("https://libraries.minecraft.net/com/mojang/text2speech/1.11.3/text2speech-1.11.3-natives-windows.jar", None),
            download_file("https://libraries.minecraft.net/com/mojang/text2speech/1.11.3/text2speech-1.11.3-sources.jar", None),
            download_file("https://libraries.minecraft.net/ca/weblite/java-objc-bridge/1.0.0/java-objc-bridge-1.0.0.jar", None),
            download_file("https://libraries.minecraft.net/ca/weblite/java-objc-bridge/1.0.0/java-objc-bridge-1.0.0-javadoc.jar", None),
            download_file("https://libraries.minecraft.net/ca/weblite/java-objc-bridge/1.0.0/java-objc-bridge-1.0.0-natives-osx.jar", None),
            download_file("https://libraries.minecraft.net/ca/weblite/java-objc-bridge/1.0.0/java-objc-bridge-1.0.0-sources.jar", None),
            download_file("https://libraries.minecraft.net/ca/weblite/java-objc-bridge/1.0.0/java-objc-bridge-1.0.0.jar", None),
            download_file("https://launcher.mojang.com/v1/objects/1952d94a0784e7abda230aae6a1e8fc0522dba99/client.jar", None),
        )
    {
        log::error!("Downloading libraries failed");
        process::exit(1);
    }
    
    env::set_current_dir(cwd)?;

    Ok(())
}

#[derive(Debug)]
pub enum AssetObjectVecType {
    Num(usize),
    Object(Value),
}

pub async fn download_mc_assets() -> Result<(), Box<dyn Error>> {
    let cwd = env::current_dir().unwrap();
    env::set_current_dir(crate::MC_LIBS_PATH.get().unwrap().clone()).unwrap();
    
    let mut index_path = env::current_dir()?;
    index_path.push("assets");
    index_path.push("indexes");
    index_path.push("1.16.json");
    let index = reqwest::get("https://launchermeta.mojang.com/v1/packages/f8e11ca03b475dd655755b945334c7a0ac2c3b43/1.16.json").await?.bytes().await?;
    fs::write(index_path, &index)?;
    
    let assets: Value = serde_json::from_str(&String::from_utf8_lossy(index.deref()))?;
    let obj = &assets["objects"];
    let mut o: Vec<Vec<AssetObjectVecType>> = Vec::new();
    use self::AssetObjectVecType::*;
    for (num, key) in obj.as_object().unwrap().keys().enumerate() {
        o.push(vec![Num(num), Object(obj.get(key).unwrap().to_owned())]);
    };
    
    for asset_obj_info in o {
        log::info!("o[0]: {:#?}", asset_obj_info[0]);
        
        match &asset_obj_info[1] {
            Object(obj) => {
                let h = obj.get("hash").unwrap();
                let filename = format!("{}/{}", &h.as_str().unwrap()[..2], &h);
                let dirname = format!("assets/objects/{}", filename);
                let url = format!("https://resources.download.minecraft.net/{}", filename);
                fs::create_dir_all(PathBuf::from(&dirname).parent().unwrap())?;
                fs::write(&dirname, reqwest::get(url).await?.bytes().await?)?;
            }
            _ => panic!("Unable to download assets")
        }
    }
    
    env::set_current_dir(cwd).unwrap();
    
    Ok(())
}
