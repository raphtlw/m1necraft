//
//  AppDelegate.swift
//  M1necraft
//
//  Created by Raphael Tang on 6/2/22.
//

import Foundation
import AppKit

class AppDelegate: NSObject, NSApplicationDelegate {
    func applicationShouldTerminateAfterLastWindowClosed(_ sender: NSApplication) -> Bool {
        return true
    }
    func applicationShouldTerminate(_ sender: NSApplication) -> NSApplication.TerminateReply {
        // Make sure all initialization is completed
        if Runtime.allowAppToTerminate {
            return .terminateNow
        } else {
            let alert = NSAlert()
            alert.messageText = "Some important processes are still running."
            alert.informativeText = "Do you really want to quit?"
            alert.alertStyle = .critical
            let yesBtn = alert.addButton(withTitle: "Yes")
            yesBtn.bezelColor = .alternateSelectedControlTextColor
            let noBtn = alert.addButton(withTitle: "No")
            noBtn.keyEquivalent = "\r"
            if alert.runModal() == .alertFirstButtonReturn {
                // terminate the app
                return .terminateNow
            }
            return .terminateLater
        }
    }
}
