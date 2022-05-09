//
//  M1necraftApp.swift
//  M1necraft
//
//  Created by Raphael Tang on 18/12/21.
//

import SwiftUI
import AppKit

@available(OSX 11.0, *)
struct M1necraftApp: App {
    @NSApplicationDelegateAdaptor(AppDelegate.self) var appDelegate
    
    @StateObject var m = ContentView.ViewModel()
    @StateObject var updater = UpdaterViewModel()
    
    var body: some Scene {
        WindowGroup {
            ContentView()
                .frame(minWidth: 600, minHeight: 400)
                .environmentObject(m)
                .environmentObject(updater)
        }.commands {
            #if DEBUG
            CommandMenu("Debug") {
                Button("Reset Data") {
                    try! Paths.dataReset()
                    m.setupStatus = .settingUp
                }
                Button("Open Minecraft") {
                    MinecraftLauncher.run()
                }
            }
            CommandGroup(after: .appInfo) {
                AppUpdaterView(updater: updater)
            }
            #endif
        }
        Settings {
            SettingsView()
        }
    }
}
