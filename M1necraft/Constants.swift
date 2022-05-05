//
//  Constants.swift
//  M1necraft
//
//  Created by Raphael Tang on 19/12/21.
//

import Foundation
import AppKit
import Glob
import Path

enum UrlValues {
    static let resourceArtifactPrefix = "https://github.com/raphtlw/m1necraft/releases/download/resources/"
    static let javaRuntime8 = "https://cdn.azul.com/zulu/bin/zulu8.58.0.13-ca-jre8.0.312-macosx_aarch64.zip"
    static let javaRuntime17 = "https://cdn.azul.com/zulu/bin/zulu17.32.13-ca-jre17.0.2-macosx_aarch64.zip"
    static let minecraftLauncher = "https://launcher.mojang.com/download/Minecraft.dmg"
}

enum Strings {
    static let launcherProfileKeyPrefix = "m1necraft-"
    static let minecraftLauncherBundleID = "com.mojang.minecraftlauncher"
}

/**
 Path values used across the whole app
 
 For now, I would assume that `data` cannot be nil, and if it happens to be,
 it means that the value was accessed too early in the app initialization process.
 Therefore, it is safe to lazily initialize these values and force the app to crash
 if they aren't set.
 
 These values are not set to change, so making it a computed value wouldn't
 really make a ton of sense. Only paths that might have their contents deleted
 during runtime would need to be computed properties.
 
 Side note: As this is a class that holds paths, all values should be of type `Path`
 and not File/Folder because we cannot assume that all paths are created on
 initialisation.
 */
struct Paths {
    private(set) static var data: P = dataInit()
    private static func dataInit() -> P {
        guard let bundleID = Bundle.main.bundleIdentifier else {
            fatalError("Bundle.main.bundleIdentifier cannot be nil!")
        }
        
        let path = P.applicationSupport/bundleID
        if !path.exists {
            try! path.mkdir()
        }
        return path
    }
    static func dataReset() throws {
        try data.delete()
        data = dataInit()
    }
    
    struct Resources {
        static let lwjgl = data/"lwjgl"
        static let lwjglnatives = lwjgl/"lwjglnatives"
        static let lwjglfatJar = lwjgl/"lwjglfat.jar"
        static let mclProfiles = data/"mcl_profiles"

        static var jre: [Int: P] {
            get throws {
                let dirs = Glob(pattern: "\(data.string)/zulu*_aarch64/zulu-*.jre")
                if dirs.indices.contains(1) {
                    return [
                        8: P(dirs.first(where: { $0.contains("zulu8") })!)!,
                        17: P(dirs.first(where: { $0.contains("zulu17") })!)!
                    ]
                } else {
                    throw AppError.glob(dirs)
                }
            }
        }
    }
    
    struct Minecraft {
        static let launcherDownload = data/"Minecraft.dmg"
        /**
         This path is not valid once the DMG is unmounted
         */
        static var launcherDownloadMounted: P {
            get throws {
                guard let path = P("/Volumes/Minecraft") else {
                    throw AppError.volumeNotMounted(launcherDownload)
                }
                return path
            }
        }
        static let launcher = data/"Minecraft.app"
        static let workDir = data/"minecraft"
        static let launcherProfiles = workDir/"launcher_profiles.json"
        static let versions = workDir/"versions"
        static let runtime = workDir/"runtime"
        static let libraries = workDir/"libraries"
        static let jre: [Int: P] = [8: runtime/"zulu-8.jre", 17: runtime/"zulu-17.jre"]
        static let lwjglnatives = workDir/"lwjglnatives"
        static let lwjglfatJar = libraries/"lwjglfat.jar"
    }
}

//class _Paths {
//    let dataDir: URL
//
//    let resLwjgl: URL
//    let resLwjglnatives: URL
//    let resLwjglfatJar: URL
//    let resMclProfiles: URL
//    let resChecksums: URL
//
//    let mclDir: URL
//    let mclLauncherProfiles: URL
//    let mclVersions: URL
//    let mclLwjglnatives: URL
//    let mclLwjglfatJar: URL
//    let mclRuntime: URL
//    let mclJre: [Int: URL]
//
//    var mclApp: Folder {
//        get throws {
//            try Folder(path: dataDir.appendingPathComponent("Minecraft.app").path)
//        }
//    }
//    var mclAppDownload: Folder {
//        get throws {
//            try Folder(path: dataDir.appendingPathComponent("Minecraft.dmg").path)
//        }
//    }
//    var mclAppDmgMounted: Folder {
//        get throws {
//            try Folder(path: "/Volumes/Minecraft")
//        }
//    }
//    var mclAppWorkDir: Folder {
//        get throws {
//            try Folder(path: dataDir.appendingPathComponent("minecraft").path)
//        }
//    }
//
//    init?() {
//        do {
//            let fileManager = FileManager()
//
//            // Program cannot function if dataDir is nil
//            self.dataDir = fileManager.urls(for: .applicationSupportDirectory,
//                                            in: .userDomainMask).first!
//                .appendingPathComponent(Bundle.main.bundleIdentifier ?? "")
//
//            // Create dataDir
//            if !fileManager.dirExists(atPath: dataDir) {
//                maybe(try fileManager.createDirectory(at: dataDir, withIntermediateDirectories: false))
//            }
//
//            resMclProfiles = dataDir.appendingPathComponent("mcl_profiles")
//            resLwjgl = dataDir.appendingPathComponent("lwjgl")
//            resLwjglnatives = resLwjgl.appendingPathComponent("lwjglnatives")
//            resLwjglfatJar = resLwjgl.appendingPathComponent("lwjglfat.jar")
//            resChecksums = dataDir.appendingPathComponent("checksums.txt")
//
//            self.mclDir = fileManager.urls(for: .applicationSupportDirectory,
//                                           in: .userDomainMask).first!
//                .appendingPathComponent("minecraft")
//
//            mclLauncherProfiles = mclDir.appendingPathComponent("launcher_profiles.json")
//            mclVersions = mclDir.appendingPathComponent("versions")
//            mclLwjglfatJar = mclDir.appendingPathComponent("libraries").appendingPathComponent("lwjglfat.jar")
//            mclLwjglnatives = mclDir.appendingPathComponent("lwjglnatives")
//            mclRuntime = mclDir.appendingPathComponent("runtime")
//            mclJre = [8: mclRuntime.appendingPathComponent("zulu-8.jre"), 17: mclRuntime.appendingPathComponent("zulu-17.jre")]
//        } catch {
//            let alert = NSAlert()
//            alert.messageText = "Encountered a problem"
//            alert.informativeText = "Failed to initialize paths. Please report this incident."
//            alert.alertStyle = .critical
//            alert.addButton(withTitle: "Quit")
//            if alert.runModal() == .alertFirstButtonReturn {
//                // terminate the app
//                Runtime.forceTerminate(nil)
//            }
//            return nil
//        }
//    }
//
//    var resJre: [Int: URL]! {
//        get throws {
//
//        }
//    }
//
//    static let global: Paths! = Paths()
//}

enum AppError: Error {
    case loremIpsum
    case glob(Glob)
    case volumeNotMounted(Path)
    case setupFailure(Error)
    
    var localizedDescription: String {
        get {
            switch(self) {
            case .loremIpsum:
                return """
                Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod tempor incididunt ut labore et dolore magna aliqua. Ut enim ad minim veniam, quis nostrud exercitation ullamco laboris nisi ut aliquip ex ea commodo consequat. Duis aute irure dolor in reprehenderit in voluptate velit esse cillum dolore eu fugiat nulla pariatur. Excepteur sint occaecat cupidatat non proident, sunt in culpa qui officia deserunt mollit anim id est laborum.
                """
            case .glob(let glob):
                return "Glob didn't find any path that matched \(glob)!"
            case .volumeNotMounted(let path):
                return "\(path.string) is not mounted!"
            case .setupFailure(let error):
                return error.localizedDescription
            }
        }
    }
}

// MARK: - JSON types
struct Empty: Encodable {}

// MARK: - Minecraft Versions
struct M1necraftVersion: Identifiable, Equatable, Comparable {
    let name: String
    private(set) var id = UUID()
    var installState: MinecraftInstallState = .notInstalled {
        didSet {
            id = UUID()
        }
    }
    
    static func < (lhs: M1necraftVersion, rhs: M1necraftVersion) -> Bool {
        return lhs.name.compare(rhs.name, options: .numeric) == .orderedAscending
    }
}

// MARK: - MinecraftLauncherProfiles JSON type
struct MinecraftLauncherProfiles: Codable {
    var profiles: [String: Profile]
    let settings: Settings?
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
        let crashAssistance,
            enableAdvanced,
            enableAnalytics,
            enableHistorical,
            enableReleases,
            enableSnapshots,
            keepLauncherOpen,
            showGameLog,
            showMenu,
            soundOn: Bool?
        let profileSorting: String?
    }
    
    static func `default`() -> Self {
        return MinecraftLauncherProfiles(profiles: [:], settings: nil, version: 3)
    }
}
