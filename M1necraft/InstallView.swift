//
//  InstallView.swift
//  M1necraft
//
//  Created by Raphael Tang on 2/2/22.
//

import SwiftUI

struct InstallView: View {
    @ObservedObject var contentViewModel: ContentView.ViewModel
    @StateObject var m = ViewModel()
    let versionRefreshTimer = Timer.publish(every: 5, on: .main, in: .common).autoconnect()
    
    var body: some View {
        VStack {
            GameVersionListView(contentViewModel: contentViewModel, m: m)
        }
        .onReceive(versionRefreshTimer, perform: { _ in
            // TODO: listen to filesystem changes in mclDir instead of refreshing every x secs
            m.refreshVersions()
        })
    }
}

extension InstallView {
    @MainActor class ViewModel: ObservableObject {
        @Published var versions = M1necraftVersion.all
        @Published var selectedMinecraftVersionID: UUID?
        
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
                    updateState(.copying)
                    
                    if Paths.Minecraft.requiredExists() == false {
                        try MinecraftLauncher.createLauncherFiles()
                    }
                    
                    ignoreError(try Paths.Resources.lwjglnatives.copy(to: Paths.Minecraft.lwjglnatives))
                    ignoreError(try Paths.Resources.lwjglfatJar.copy(to: Paths.Minecraft.lwjglfatJar))
                    ignoreError(try Paths.Resources.mclProfiles.join(profileName).copy(into: Paths.Minecraft.versions))
                    ignoreError(try Paths.Resources.jre[version.javaVersion]!.copy(into: Paths.Minecraft.runtime, overwrite: true))
                } catch {
                    print("Copying files to Minecraft failed.")
                    print(error.localizedDescription)
                }
                
                // add launcher profile
                do {
                    updateState(.addingProfile)
                    var launcherProfilesJson = try String(contentsOf: Paths.Minecraft.launcherProfiles.url)
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
                            javaDir: Paths.Minecraft.jre[version.javaVersion]!.join("Contents/Home/bin/java").string,
                            lastUsed: nil)
                        launcherProfiles.profiles[newLauncherProfileKey] = newLauncherProfile
                        jsonEncoder.outputFormatting = .prettyPrinted
                        launcherProfilesJson = try String(data: jsonEncoder.encode(launcherProfiles), encoding: .utf8)!
                        try launcherProfilesJson.write(to: Paths.Minecraft.launcherProfiles.url, atomically: false, encoding: .utf8)
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
        InstallView(contentViewModel: ContentView.ViewModel())
    }
}
