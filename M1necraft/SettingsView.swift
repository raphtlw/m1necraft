//
//  SettingsView.swift
//  M1necraft
//
//  Created by Raphael Tang on 13/2/22.
//

import SwiftUI

struct SettingsView: View {
    @ObservedObject var m: ContentView.ViewModel
    var showCloseButton = false
    
    var body: some View {
        VStack(alignment: .leading, spacing: 5) {
            if showCloseButton {
                HStack {
                    Spacer()
                    CloseButton {
                        m.activeSheet = nil
                    }
                }
            }
            
            Text("Settings")
                .font(.title.bold())
                .padding(.bottom, 8)
            Form {
                Button("Reset all data") {
                    try! Paths.global.resetDataDir()
                    try! Paths.global.mclLwjglnatives.folder?.delete()
                    try! Paths.global.mclLwjglfatJar.file?.delete()
                    try! Paths.global.mclJre.forEach { url in
                        try url.value.folder?.delete()
                    }
                    
                    // delete all installed versions
                    try! supportedVersions.forEach { version in
                        try version.isInstalledAt?.folder?.delete()
                    }
                    
                    let jsonEncoder = JSONEncoder()
                    let jsonDecoder = JSONDecoder()
                    
                    var launcherProfiles = try! jsonDecoder.decode(MinecraftLauncherProfiles.self, from: try! String(contentsOf: Paths.global.mclLauncherProfiles).data(using: .utf8)!)
                    launcherProfiles.profiles.forEach { profileItem in
                        if profileItem.key.hasPrefix("m1necraft") {
                            launcherProfiles.profiles.removeValue(forKey: profileItem.key)
                        }
                    }
                    
                    try! String(data: jsonEncoder.encode(launcherProfiles), encoding: .utf8)?.write(to: Paths.global.mclLauncherProfiles, atomically: false, encoding: .utf8)
                    
                    m.setupStatus = .settingUp
                    m.activeSheet = nil
                }
            }
        }
        .padding(.all, 20)
        .frame(width: 500)
    }
}

struct SettingsView_Previews: PreviewProvider {
    static var previews: some View {
        SettingsView(m: ContentView.ViewModel())
    }
}
