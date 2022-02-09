//
//  AppCompat.swift
//  M1necraft
//
//  Created by Raphael Tang on 6/2/22.
//

import Foundation
import AppKit
import SwiftUI

struct M1necraftAppCompat {
    static let appDelegate = AppDelegateCompat()
    
    static func main() {
        NSApplication.shared.setActivationPolicy(.regular)
        
        let nib = NSNib(nibNamed: NSNib.Name("M1necraft"), bundle: Bundle.main)
        nib?.instantiate(withOwner: NSApplication.shared, topLevelObjects: nil)
        
        NSApp.delegate = appDelegate
        NSApp.activate(ignoringOtherApps: true)
        NSApp.run()
    }
}

class AppDelegateCompat: AppDelegate {
    var window: NSWindow!
    
    @MainActor func application(_ application: NSApplication) {
        let contentView = ContentView(m: ContentView.ViewModel())
        
        window = NSWindow(
            contentRect: NSRect(x: 0, y: 0, width: 480, height: 300),
            styleMask: [.titled, .closable, .miniaturizable, .resizable, .fullSizeContentView],
            backing: .buffered, defer: false)
        
        window.title = "M1necraft (compatibility)"
        window.isReleasedWhenClosed = false
        window.center()
        window.setFrameAutosaveName("Main Window")
        window.contentView = NSHostingView(rootView: contentView)
        window.makeKeyAndOrderFront(nil)
    }
}
