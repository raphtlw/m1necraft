use std::error::Error;
use std::io::{Write, self};
use std::ops::Deref;
use std::path::PathBuf;
use std::process::Stdio;
use std::{env, fs, process};

use reqwest::IntoUrl;
use serde_json::{Value, json};
use indoc::formatdoc;

use crate::{MC_LIBS_PATH, APP_DATA_DIR};
use crate::{Message};
use crate::config::{MinecraftCredentials, Config};

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
    
    Message::dispatch(Message::SetSetupProgress(Some(String::from("Downloading Minecraft libraries")), 0.1));
    
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
    
    Message::dispatch(Message::SetSetupProgress(None, 1.0));
    
    env::set_current_dir(cwd)?;

    Ok(())
}

#[derive(Debug)]
pub enum AssetObjectVecType {
    Num(usize),
    Object(Value),
}

/// Downloads all assets required for Minecraft to run, like objects, textures,
/// etc.
pub async fn download_mc_assets() -> Result<(), Box<dyn Error>> {
    let cwd = env::current_dir().unwrap();
    env::set_current_dir(crate::MC_LIBS_PATH.get().unwrap().clone()).unwrap();
    
    Message::dispatch(Message::SetSetupProgress(Some(String::from("Downloading Minecraft assets")), 0.1));
    
    let mut index_path = env::current_dir()?;
    index_path.push("assets");
    index_path.push("indexes");
    index_path.push("1.16.json");
    let index = reqwest::get("https://launchermeta.mojang.com/v1/packages/f8e11ca03b475dd655755b945334c7a0ac2c3b43/1.16.json").await?.bytes().await?;
    fs::write(index_path, &index)?;
    
    Message::dispatch(Message::SetSetupProgress(None, 0.3));
    
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
    
    Message::dispatch(Message::SetSetupProgress(None, 1.0));
    
    env::set_current_dir(cwd).unwrap();
    
    Ok(())
}

/// Launches Minecraft. Only works for version 1.16.4 for now.
/// Should be called only after login credentials have been saved,
/// because this function will authenticate with Minecraft servers.
/// 
/// TODO: This probably does not work, should fix.
pub async fn launch() -> Result<(), Box<dyn Error>> {
    let _cwd = env::current_dir()?;
    env::set_current_dir(MC_LIBS_PATH.get().unwrap().clone())?;
    
    let mc_libs_dir = MC_LIBS_PATH.get().unwrap().clone();
    let config = Config::read().unwrap();
    let auth_data = authenticate(config.minecraft_creds).await?;
    
    let launch_script = formatdoc! {r#"
        mainClass net.minecraft.client.main.Main
        param --version
        param MultiMC5
        param --assetIndex
        param 1.16
        param --userType
        param mojang
        param --versionType
        param release
        windowTitle MultiMC: Working
        windowParams 854x480
        traits XR:Initial
        traits FirstThreadOnMacOS
        launcher onesix
        param --gameDir
        param {game_dir}
        param --assetsDir
        param {assets_dir}
        param --accessToken
        param {access_token}
        sessionId token:{access_token}
        param --username
        param {username}
        userName {username}
        param --uuid
        param {uuid}
        cp {libraries_dir}/lwjgljars.jar
        cp {libraries_dir}/patchy-1.1.jar
        cp {libraries_dir}/project/oshi-core/1.1/oshi-core-1.1.jar
        cp {libraries_dir}/jna-4.4.0.jar
        cp {libraries_dir}/platform-3.4.0.jar
        cp {libraries_dir}/icu4j-66.1.jar
        cp {libraries_dir}/javabridge-1.0.22.jar
        cp {libraries_dir}/jopt-simple-5.0.3.jar
        cp {libraries_dir}/netty-all-4.1.25.Final.jar
        cp {libraries_dir}/guava-21.0.jar
        cp {libraries_dir}/commons-lang3-3.5.jar
        cp {libraries_dir}/commons-io-2.5.jar
        cp {libraries_dir}/commons-codec-1.10.jar
        cp {libraries_dir}/brigadier-1.0.17.jar
        cp {libraries_dir}/datafixerupper-4.0.26.jar
        cp {libraries_dir}/gson-2.8.0.jar
        cp {libraries_dir}/authlib-2.0.27.jar
        cp {libraries_dir}/commons-compress-1.8.1.jar
        cp {libraries_dir}/httpclient-4.3.3.jar
        cp {libraries_dir}/commons-logging-1.1.3.jar
        cp {libraries_dir}/httpcore-4.3.2.jar
        cp {libraries_dir}/fastutil-8.2.1.jar
        cp {libraries_dir}/log4j-api-2.8.1.jar
        cp {libraries_dir}/log4j-core-2.8.1.jar
        cp {libraries_dir}/text2speech-1.11.3.jar
        cp {libraries_dir}/java-objc-bridge-1.0.0.jar
        cp {libraries_dir}/minecraft-1.16.4-client.jar
        ext {libraries_dir}/java-objc-bridge-1.0.0-natives-osx.jar
        natives NO_NATIVES
        launch
        "#,
        game_dir = mc_libs_dir.join("minecraft").to_string_lossy(),
        assets_dir = mc_libs_dir.join("assets").to_string_lossy(),
        access_token = auth_data.auth_token,
        username = auth_data.username,
        uuid = auth_data.uuid,
        libraries_dir = mc_libs_dir.join("libraries").to_string_lossy(),
    };
    
    let java_args = formatdoc! {r#"
        -Dorg.lwjgl.librarypath={wd}/lwjglnatives
        -Xdock:icon=icon.png
        -Xdock:name=AppleSiliconMinecraft
        -XstartOnFirstThread
        -Xms409m
        -Xmx2048m
        -Duser.language=en
        -cp {wd}/NewLaunch.jar:{wd}/libraries/lwjglfat.jar:{wd}/libraries/patchy-1.1.jar:{wd}/libraries/project/oshi-core/1.1/oshi-core-1.1.jar:{wd}/libraries/jna-4.4.0.jar:{wd}/libraries/platform-3.4.0.jar:{wd}/libraries/icu4j-66.1.jar:{wd}/libraries/javabridge-1.0.22.jar:{wd}/libraries/jopt-simple-5.0.3.jar:{wd}/libraries/netty-all-4.1.25.Final.jar:{wd}/libraries/guava-21.0.jar:{wd}/libraries/commons-lang3-3.5.jar:{wd}/libraries/commons-io-2.5.jar:{wd}/libraries/commons-codec-1.10.jar:{wd}/libraries/brigadier-1.0.17.jar:{wd}/libraries/datafixerupper-4.0.26.jar:{wd}/libraries/gson-2.8.0.jar:{wd}/libraries/authlib-2.0.27.jar:{wd}/libraries/commons-compress-1.8.1.jar:{wd}/libraries/httpclient-4.3.3.jar:{wd}/libraries/commons-logging-1.1.3.jar:{wd}/libraries/httpcore-4.3.2.jar:{wd}/libraries/fastutil-8.2.1.jar:{wd}/libraries/log4j-api-2.8.1.jar:{wd}/libraries/log4j-core-2.8.1.jar:{wd}/libraries/text2speech-1.11.3.jar:{wd}/libraries/java-objc-bridge-1.0.0.jar:{wd}/libraries/minecraft-1.16.4-client.jar:{wd}/libraries/java-objc-bridge-1.0.0-natives-osx.jar
        org.multimc.EntryPoint
        "#,
        wd = MC_LIBS_PATH.get().unwrap().clone().to_string_lossy()
    }.replace("\n", " ");
    let launcher_process = process::Command::new(MC_LIBS_PATH.get().unwrap().clone().join("zulu-11.jdk/Contents/Home/bin/java")).arg(java_args).stdin(Stdio::piped()).stdout(Stdio::piped()).spawn().expect("Launcher command failed to start");
    let mut launcher_process_stdin = launcher_process.stdin.unwrap();
    launcher_process_stdin.write_all(launch_script.as_bytes())?;
    let mut mclog_file = fs::File::create(APP_DATA_DIR.get().unwrap().clone().join("mclog.log"))?;
    io::copy(&mut launcher_process.stdout.unwrap(), &mut mclog_file)?;
    
    Ok(())
}

#[derive(Clone)]
pub struct AuthResult {
    uuid: String,
    username: String,
    auth_token: String,
}

pub async fn authenticate(creds: MinecraftCredentials) -> Result<AuthResult, Box<dyn Error>> {
    let auth_json = json!({
        "agent": {
            "name": "Minecraft",
            "version": 1
        },
        "clientToken": "client identifier",
        "requestUser": true,
        "username": creds.username,
        "password": creds.password,
    });
    
    let req_client = reqwest::Client::new();
    let resp = req_client.post("https://authserver.mojang.com/authenticate").body(auth_json.to_string()).send().await?.text().await?;
    let resp: Value = serde_json::from_str(&resp)?;
    
    let result = AuthResult {
        uuid: resp.get("selectedProfile").unwrap().get("id").unwrap().as_str().unwrap().to_string(),
        username: resp.get("selectedProfile").unwrap().get("name").unwrap().as_str().unwrap().to_string(),
        auth_token: resp.get("accessToken").unwrap().as_str().unwrap().to_string()
    };
    
    log::info!("uuid: {}", result.uuid);
    log::info!("username: {}", result.username);
    log::info!("auth_token: {}", result.auth_token);
    
    Ok(result)
}
