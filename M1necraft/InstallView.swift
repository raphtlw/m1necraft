//
//  InstallView.swift
//  M1necraft
//
//  Created by Raphael Tang on 2/2/22.
//

import SwiftUI
import Files

struct InstallView: View {
    @StateObject var m = ViewModel()
    let versionRefreshTimer = Timer.publish(every: 5, on: .main, in: .common).autoconnect()
    
    var body: some View {
        VStack {
            GameVersionListView(m: m)
        }
        .sheet(item: $m.activeSheet) { $0.modalView(viewModel: m) }
        .onReceive(versionRefreshTimer, perform: { _ in
            // TODO: listen to filesystem changes in mclDir instead of refreshing every x secs
            m.refreshVersions()
        })
    }
}

extension InstallView {
    @MainActor class ViewModel: ObservableObject {
        @Published var versions = supportedVersions
        @Published var selectedMinecraftVersionID: UUID?
        @Published var activeSheet: Sheet?
        
        let jsonEncoder = JSONEncoder()
        let fileManager = FileManager()
        
        func refreshVersions() {
            versions.indices.forEach {
                if let path = versions[$0].isInstalledAt {
                    versions[$0].installState = .installed(path)
                } else {
                    versions[$0].installState = .notInstalled
                }
            }
        }
        
        func installVersion(version: M1necraftVersion, updateState: (_ installationStep: InstallationStep) -> Void) {
            refreshVersions()
            
            if case .notInstalled = version.installState {
                updateState(.starting)
                let profileName = "\(version.name)-arm"
                
                // add files to minecraft
                do {
                    updateState(.copying(destination: Paths.global.mclDir.path))
                    try Paths.global.resLwjglnatives.folder!.copyReplace(to: Paths.global.mclDir.folder!)
                    updateState(.copying(destination: Paths.global.mclLwjglfatJar.path))
                    try Paths.global.resLwjglfatJar.file!.copyReplace(to: Paths.global.mclDir.folder!.createSubfolderIfNeeded(withName: "libraries"))
                    updateState(.copying(destination: Paths.global.mclVersions.path))
                    try Paths.global.resMclProfiles.appendingPathComponent(profileName).folder!.copyReplace(to: Paths.global.mclVersions.folder!)
                    updateState(.copying(destination: Paths.global.mclRuntime.path))
                    try Paths.global.resJre[version.javaVersion]!.folder!.copyReplace(to: Paths.global.mclDir.folder!.createSubfolderIfNeeded(withName: "runtime"))
                } catch {
                    print("Copying files to Minecraft failed.")
                    print(error.localizedDescription)
                }
                
                // add launcher profile
                do {
                    updateState(.addingProfile)
                    let launcherProfilesJson = try String(contentsOf: Paths.global.mclLauncherProfiles)
                    var launcherProfiles = try JSONDecoder().decode(MinecraftLauncherProfiles.self, from: launcherProfilesJson.data(using: .utf8)!)
                    
                    // construct new launcher profile
                    let newLauncherProfileKey = Strings.launcherProfileKeyPrefix.appending(version.name)
                    if launcherProfiles.profiles[newLauncherProfileKey] == nil {
                        print("Launcher profile does not exist, creating...")
                        let localISOFormatter = ISO8601DateFormatter()
                        localISOFormatter.timeZone = .current
                        let newLauncherProfile = MinecraftLauncherProfiles.Profile(
                            created: localISOFormatter.string(from: Date()),
                            icon: "Grass",
                            lastVersionID: profileName,
                            name: "M1necraft",
                            type: "custom",
                            javaDir: Paths.global.mclJre[version.javaVersion]!.appendingPathComponent("Contents/Home/bin/java").path,
                            lastUsed: nil)
                        launcherProfiles.profiles[newLauncherProfileKey] = newLauncherProfile
                        jsonEncoder.outputFormatting = .prettyPrinted
                        let newLauncherProfilesJson = try String(data: jsonEncoder.encode(launcherProfiles), encoding: .utf8)
                        try newLauncherProfilesJson?.write(to: Paths.global.mclLauncherProfiles, atomically: false, encoding: .utf8)
                    }
                } catch {
                    print("Adding profile to launcher failed.")
                    print(error.localizedDescription)
                }
                
                // finish up
                updateState(.finishing)
                refreshVersions()
            }
        }
    }
}

struct InstallView_Previews: PreviewProvider {
    static var previews: some View {
        InstallView()
    }
}
