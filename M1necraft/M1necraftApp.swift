//
//  M1necraftApp.swift
//  M1necraft
//
//  Created by Raphael Tang on 18/12/21.
//

import SwiftUI

@main
struct M1necraftApp: App {
    var body: some Scene {
        WindowGroup {
            #if os(macOS)
            ContentView().frame(minWidth: 600, maxWidth: 600, minHeight: 400, maxHeight: 400)
            #else
            ContentView()
            #endif
        }
    }
}
