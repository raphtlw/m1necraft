//
//  LauncherView.swift
//  M1necraft
//
//  Created by Raphael Tang on 19/12/21.
//

import SwiftUI

struct LauncherView: View {
    @ObservedObject var m: ContentView.ViewModel
    
    var body: some View {
        Group {
            if m.mcLibsInstalled {
                VStack {
                    
                }
            } else {
                VStack {
                    Text("Setting up and installing Minecraft for you.")
                    Text("Please wait until the process is completed.")
                    ProgressView(m.currentSetupStatus, value: m.currentSetupProgress.fractionCompleted)
                }
            }
        }.padding()
    }
}

struct LauncherView_Previews: PreviewProvider {
    static var previews: some View {
        LauncherView(m: ContentView.ViewModel())
    }
}
