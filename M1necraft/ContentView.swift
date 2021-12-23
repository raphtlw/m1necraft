//
//  ContentView.swift
//  M1necraft
//
//  Created by Raphael Tang on 18/12/21.
//

import SwiftUI
import KeychainSwift
import Alamofire
import ZIPFoundation

struct ContentView: View {
    @AppStorage("signedIn") private var signedIn = false
    @StateObject var m = ViewModel()
    
    var body: some View {
        Group {
            if !signedIn {
                LoginView(m: m)
            } else {
                LauncherView(m: m)
            }
        }
        .onAppear {
            self.m.onViewAppear()
        }
    }
}

extension ContentView {
    class ViewModel: ObservableObject {
        @AppStorage("minecraftUsername") var minecraftUsername = ""
        @Published var minecraftPassword = ""
        @AppStorage("mcLibsInstalled") var mcLibsInstalled = false
        @Published var currentSetupStatus = ""
        @Published var currentSetupProgress = Progress()
        
        func onViewAppear() async {
            // try getting the minecraft password from the keychain
            if let minecraftPasswordUnwrapped = KeychainSwift().get("m1necraft_password") {
                self.minecraftPassword = minecraftPasswordUnwrapped
            }
            
            // also start fetching mc_libs if not downloaded yet
            debugPrint(Paths.dataDir)
            
            let fileManager = FileManager()
            let mcLibsDownloadDestination = Paths.dataDir.appendingPathComponent("mc_libs.zip")
            if !fileManager.fileExists(atPath: mcLibsDownloadDestination.path) {
                self.currentSetupStatus = "Downloading libraries"
                
                let downloadReqDestination: DownloadRequest.Destination = { _, _ in
                    return (mcLibsDownloadDestination, [.removePreviousFile, .createIntermediateDirectories])
                }
                AF.download(UrlValues.mcLibsUrl, to: downloadReqDestination)
                    .downloadProgress { progress in
                        self.currentSetupProgress = progress
                    }
                    .responseURL { response in
                        self.extractLib(from: mcLibsDownloadDestination, to: Paths.dataDir)
                    }
            }
            
            // download Minecraft Client & client libraries
            if !fileManager.fileExists(atPath: Paths.minecraftClientJar.path) {
                self.currentSetupStatus = "Downloading Minecraft client"
                
                AF.download(UrlValues.minecraftClientJar, to: { _, _ in
                    return (Paths.minecraftClientJar, [.removePreviousFile, .createIntermediateDirectories])
                })
                    .downloadProgress { progress in
                        self.currentSetupProgress = progress
                    }
                    .responseURL { response in
                        
                    }
                
                self.currentSetupStatus = "Downloading Minecraft client libraries"
                
                for libraryUrl in UrlValues.minecraftClientLibraries {
                    AF.download(libraryUrl, to: { _, _ in
                        return (Paths.minecraftLibrariesDir.appendingPathComponent(URL(string: libraryUrl)!.lastPathComponent), [.removePreviousFile, .createIntermediateDirectories])
                    })
                        .downloadProgress { progress in
                            self.currentSetupProgress = progress
                        }
                        .responseURL { response in
                            
                        }
                }
            }
        }
        
        func extractLib(from: URL, to: URL) {
            self.currentSetupStatus = "Decompressing libraries"
            
            do {
                let fileManager = FileManager()
                try fileManager.createDirectory(at: to, withIntermediateDirectories: true, attributes: nil)
                try fileManager.unzipItem(at: from, to: to, progress: self.currentSetupProgress)
            } catch {
                print("Error extracting library")
                print(error.localizedDescription)
            }
        }
    }
}

struct ContentView_Previews: PreviewProvider {
    static var previews: some View {
        ContentView()
    }
}
