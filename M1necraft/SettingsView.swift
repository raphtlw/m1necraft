//
//  SettingsView.swift
//  M1necraft
//
//  Created by Raphael Tang on 13/2/22.
//

import SwiftUI

struct SettingsView: View {
    @EnvironmentObject var m: ContentView.ViewModel
    @EnvironmentObject var updater: UpdaterViewModel
    
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
                    do {
                        try Util.resetData()
                        // terminate the app
                        Runtime.forceTerminate(self)
                    } catch {
                        print(error)
                        
                        let alert = NSAlert()
                        alert.messageText = "Resetting data failed"
                        alert.informativeText = "Something went wrong. Please contact the developer."
                        alert.alertStyle = .critical
                        let quitBtn = alert.addButton(withTitle: "Quit")
                        quitBtn.keyEquivalent = "\r"
                        if alert.runModal() == .alertFirstButtonReturn {
                            Runtime.forceTerminate(self)
                        }
                    }
                }
            }
        }
        .padding(.all, 20)
        .frame(width: 500)
    }
}

struct SettingsView_Previews: PreviewProvider {
    static var previews: some View {
        SettingsView()
    }
}
