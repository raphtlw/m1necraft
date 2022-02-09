//
//  ModInstallHelpView.swift
//  M1necraft
//
//  Created by Raphael Tang on 7/2/22.
//

import SwiftUI

struct ModInstallHelpView: View {
    @StateObject var m: InstallView.ViewModel
    
    var body: some View {
        VStack(alignment: .leading, spacing: 8) {
            HStack {
                Spacer()
                CloseButton {
                    m.activeSheet = nil
                }
            }
            
            Text("Information")
                .font(.title.bold())
            Text("""
            Installing Fabric and Forge in M1necraft isn't supported yet.
            Please install them from their respective websites, just like how you would normally, by installing the Fabric Installer.jar file, and running it, etc.
            """)
        }
        .padding(.all, 20)
        .frame(width: 500)
    }
}

struct ModInstallHelpView_Previews: PreviewProvider {
    static var previews: some View {
        ModInstallHelpView(m: InstallView.ViewModel())
    }
}
