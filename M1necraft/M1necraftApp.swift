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
    
    var body: some Scene {
        WindowGroup {
            ContentView(m: m).frame(minWidth: 600, minHeight: 400)
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
            #endif
        }
        Settings {
            SettingsView(m: m)
        }
    }
}
