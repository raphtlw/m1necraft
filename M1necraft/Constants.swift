//
//  Constants.swift
//  M1necraft
//
//  Created by Raphael Tang on 19/12/21.
//

import Foundation
import AppKit
import Glob

enum UrlValues {
    static let resourceArtifactPrefix = "https://github.com/raphtlw/m1necraft/releases/download/resources/"
    static let javaRuntime8 = "https://cdn.azul.com/zulu/bin/zulu8.58.0.13-ca-jre8.0.312-macosx_aarch64.zip"
    static let javaRuntime17 = "https://cdn.azul.com/zulu/bin/zulu17.32.13-ca-jre17.0.2-macosx_aarch64.zip"
}

enum Strings {
    static let launcherProfileKeyPrefix = "m1necraft-"
    static let minecraftLauncherBundleID = "com.mojang.minecraftlauncher"
}

class Paths {
    let dataDir: URL
    
    let resLwjgl: URL
    let resLwjglnatives: URL
    let resLwjglfatJar: URL
    let resMclProfiles: URL
    let resChecksums: URL
    
    let mclDir: URL
    let mclLauncherProfiles: URL
    let mclVersions: URL
    let mclLwjglnatives: URL
    let mclLwjglfatJar: URL
    let mclRuntime: URL
    let mclJre: [Int: URL]
    
    static func initFail() {
        let alert = NSAlert()
        alert.messageText = "Encountered a problem"
        alert.informativeText = "Failed to initialize paths. Please report this incident."
        alert.alertStyle = .critical
        alert.addButton(withTitle: "Quit")
        if alert.runModal() == .alertFirstButtonReturn {
            // terminate the app
            NSApplication.shared.terminate(nil)
        }
    }

    init?() {
        let fileManager = FileManager()
        
        // Program cannot function if dataDir is nil
        guard let _dataDir = fileManager.urls(for: .applicationSupportDirectory, in: .userDomainMask).first?.appendingPathComponent(Bundle.main.bundleIdentifier ?? "") else {
            Self.initFail()
            return nil
        }
        
        self.dataDir = _dataDir
        
        // Create dataDir
        if !fileManager.dirExists(atPath: dataDir) {
            maybe(try fileManager.createDirectory(at: dataDir, withIntermediateDirectories: false))
        }
        
        resMclProfiles = dataDir.appendingPathComponent("mcl_profiles")
        resLwjgl = dataDir.appendingPathComponent("lwjgl")
        resLwjglnatives = resLwjgl.appendingPathComponent("lwjglnatives")
        resLwjglfatJar = resLwjgl.appendingPathComponent("lwjglfat.jar")
        resChecksums = dataDir.appendingPathComponent("checksums.txt")
        
        guard let _mclDir = fileManager.urls(for: .applicationSupportDirectory, in: .userDomainMask).first?.appendingPathComponent("minecraft") else {
            Self.initFail()
            return nil
        }
        
        self.mclDir = _mclDir
        
        mclLauncherProfiles = mclDir.appendingPathComponent("launcher_profiles.json")
        mclVersions = mclDir.appendingPathComponent("versions")
        mclLwjglfatJar = mclDir.appendingPathComponent("libraries").appendingPathComponent("lwjglfat.jar")
        mclLwjglnatives = mclDir.appendingPathComponent("lwjglnatives")
        mclRuntime = mclDir.appendingPathComponent("runtime")
        mclJre = [8: mclRuntime.appendingPathComponent("zulu-8.jre"), 17: mclRuntime.appendingPathComponent("zulu-17.jre")]
    }
    
    var resJre: [Int: URL]! {
        get {
            let dirs = Glob(pattern: "\(dataDir.path)/zulu*_aarch64/zulu-*.jre")
            if dirs.indices.contains(1) {
                return [8: URL(fileURLWithPath: dirs.first(where: { $0.contains("zulu8") })!, isDirectory: true), 17: URL(fileURLWithPath: dirs.first(where: { $0.contains("zulu17") })!, isDirectory: true)]
            } else {
                print("Glob didn't find any path that matched zulu*...!")
                Self.initFail()
                return nil // this will never
            }
        }
    }

    static let global: Paths! = Paths()
}

enum AppError: Error {
    case pathInitError(String)
}

// MARK: - Minecraft Versions
struct M1necraftVersion: Identifiable, Equatable {
    let name: String
    private(set) var id = UUID()
    var installState: MinecraftInstallState = .notInstalled {
        didSet {
            id = UUID()
        }
    }
}

#if DEBUG
let supportedVersions = [
    M1necraftVersion(name: "1.18.1"),
    M1necraftVersion(name: "1.18"),
    M1necraftVersion(name: "1.17.1"),
    M1necraftVersion(name: "1.17"),
    M1necraftVersion(name: "1.16.5"),
    M1necraftVersion(name: "1.16.4"),
    M1necraftVersion(name: "1.16.3"),
    M1necraftVersion(name: "1.16.2"),
    M1necraftVersion(name: "1.16.1"),
    M1necraftVersion(name: "1.16"),
    M1necraftVersion(name: "1.14.1"),
    M1necraftVersion(name: "1.12.2")
]
#else
let supportedVersions = [
    M1necraftVersion(name: "1.18.1"),
    M1necraftVersion(name: "1.18"),
    M1necraftVersion(name: "1.17.1"),
    M1necraftVersion(name: "1.17"),
    M1necraftVersion(name: "1.16.5"),
    M1necraftVersion(name: "1.16.4"),
    M1necraftVersion(name: "1.16.3"),
    M1necraftVersion(name: "1.16.2"),
    M1necraftVersion(name: "1.16.1"),
    M1necraftVersion(name: "1.16")
]
#endif

// MARK: - MinecraftLauncherProfiles JSON type
struct MinecraftLauncherProfiles: Codable {
    var profiles: [String: Profile]
    let settings: Settings
    let version: Int
    
    struct Profile: Codable {
        let created, icon, lastVersionID: String
        let name, type: String
        let javaDir, lastUsed: String?

        enum CodingKeys: String, CodingKey {
            case created, icon
            case lastVersionID = "lastVersionId"
            case name, type, javaDir, lastUsed
        }
    }

    struct Settings: Codable {
        let crashAssistance, enableAdvanced, enableAnalytics, enableHistorical: Bool
        let enableReleases, enableSnapshots, keepLauncherOpen: Bool
        let profileSorting: String
        let showGameLog, showMenu, soundOn: Bool
    }
}
