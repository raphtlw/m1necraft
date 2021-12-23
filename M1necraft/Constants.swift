//
//  Constants.swift
//  M1necraft
//
//  Created by Raphael Tang on 19/12/21.
//

import Foundation

enum UrlValues {
    static let mcLibsUrl = "https://github.com/raphtlw/m1necraft/releases/download/resources/mc_libs.zip"
    static let minecraftClientJar = "https://launcher.mojang.com/v1/objects/1952d94a0784e7abda230aae6a1e8fc0522dba99/client.jar"
    static let minecraftClientLibraries = [
        "https://libraries.minecraft.net/com/mojang/patchy/1.1/patchy-1.1.jar",
        "https://libraries.minecraft.net/oshi-project/oshi-core/1.1/oshi-core-1.1.jar",
        "https://libraries.minecraft.net/net/java/dev/jna/jna/4.4.0/jna-4.4.0.jar",
        "https://libraries.minecraft.net/net/java/dev/jna/platform/3.4.0/platform-3.4.0.jar",
        "https://libraries.minecraft.net/com/ibm/icu/icu4j/66.1/icu4j-66.1.jar",
        "https://libraries.minecraft.net/com/mojang/javabridge/1.0.22/javabridge-1.0.22.jar",
        "https://libraries.minecraft.net/net/sf/jopt-simple/jopt-simple/5.0.3/jopt-simple-5.0.3.jar",
        "https://libraries.minecraft.net/io/netty/netty-all/4.1.25.Final/netty-all-4.1.25.Final.jar",
        "https://libraries.minecraft.net/com/google/guava/guava/21.0/guava-21.0.jar",
        "https://libraries.minecraft.net/org/apache/commons/commons-lang3/3.5/commons-lang3-3.5.jar",
        "https://libraries.minecraft.net/commons-io/commons-io/2.5/commons-io-2.5.jar",
        "https://libraries.minecraft.net/commons-codec/commons-codec/1.10/commons-codec-1.10.jar",
        "https://libraries.minecraft.net/com/mojang/brigadier/1.0.17/brigadier-1.0.17.jar",
        "https://libraries.minecraft.net/com/mojang/datafixerupper/4.0.26/datafixerupper-4.0.26.jar",
        "https://libraries.minecraft.net/com/google/code/gson/gson/2.8.0/gson-2.8.0.jar",
        "https://libraries.minecraft.net/com/mojang/authlib/2.0.27/authlib-2.0.27.jar",
        "https://libraries.minecraft.net/org/apache/commons/commons-compress/1.8.1/commons-compress-1.8.1.jar",
        "https://libraries.minecraft.net/org/apache/httpcomponents/httpclient/4.3.3/httpclient-4.3.3.jar",
        "https://libraries.minecraft.net/commons-logging/commons-logging/1.1.3/commons-logging-1.1.3.jar",
        "https://libraries.minecraft.net/org/apache/httpcomponents/httpcore/4.3.2/httpcore-4.3.2.jar",
        "https://libraries.minecraft.net/it/unimi/dsi/fastutil/8.2.1/fastutil-8.2.1.jar",
        "https://libraries.minecraft.net/org/apache/logging/log4j/log4j-api/2.8.1/log4j-api-2.8.1.jar",
        "https://libraries.minecraft.net/org/apache/logging/log4j/log4j-core/2.8.1/log4j-core-2.8.1.jar",
        "https://libraries.minecraft.net/com/mojang/text2speech/1.11.3/text2speech-1.11.3.jar",
        "https://libraries.minecraft.net/com/mojang/text2speech/1.11.3/text2speech-1.11.3.jar",
        "https://libraries.minecraft.net/com/mojang/text2speech/1.11.3/text2speech-1.11.3-natives-linux.jar",
        "https://libraries.minecraft.net/com/mojang/text2speech/1.11.3/text2speech-1.11.3-natives-windows.jar",
        "https://libraries.minecraft.net/com/mojang/text2speech/1.11.3/text2speech-1.11.3-sources.jar",
        "https://libraries.minecraft.net/ca/weblite/java-objc-bridge/1.0.0/java-objc-bridge-1.0.0.jar",
        "https://libraries.minecraft.net/ca/weblite/java-objc-bridge/1.0.0/java-objc-bridge-1.0.0-javadoc.jar",
        "https://libraries.minecraft.net/ca/weblite/java-objc-bridge/1.0.0/java-objc-bridge-1.0.0-natives-osx.jar",
        "https://libraries.minecraft.net/ca/weblite/java-objc-bridge/1.0.0/java-objc-bridge-1.0.0-sources.jar",
        "https://libraries.minecraft.net/ca/weblite/java-objc-bridge/1.0.0/java-objc-bridge-1.0.0.jar",
        "https://launcher.mojang.com/v1/objects/1952d94a0784e7abda230aae6a1e8fc0522dba99/client.jar"
    ]
}

enum Paths {
    static let dataDir = FileManager.default.urls(for: .applicationSupportDirectory, in: .userDomainMask).first!
    static let mcLibsDir = dataDir.appendingPathComponent("mc_libs")
    static let minecraftLibrariesDir = mcLibsDir.appendingPathComponent("libraries")
    static let minecraftClientJar = minecraftLibrariesDir.appendingPathComponent("minecraft-1.16.4-client.jar")
}
