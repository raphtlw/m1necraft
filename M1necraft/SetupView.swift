//
//  SetupView.swift
//  M1necraft
//
//  Created by Raphael Tang on 2/2/22.
//

import SwiftUI
import Alamofire
import ZIPFoundation

struct SetupView: View {
    @StateObject var m = ViewModel()
    @ObservedObject var contentViewModel: ContentView.ViewModel
    
    var body: some View {
        if #available(macOS 12, *) {
            VStack {
                innerView()
            }.task {
                await preventTerminate {
                    // Delete existing libs, everytime this task is run, resources should be re-downloaded.
                    maybe(try Paths.global.resetDataDir())
                    await m.downloadLibs()
                    contentViewModel.setupStatus = .completed
                }
            }.background {
                VisualEffectView(blendingMode: .behindWindow).scaledToFill()
            }
        } else {
            VStack {
                innerView()
            }.onAppear {
                Task {
                    await preventTerminate {
                        // Delete existing libs, everytime this task is run, resources should be re-downloaded.
                        maybe(try Paths.global.resetDataDir())
                        await m.downloadLibs()
                        contentViewModel.setupStatus = .completed
                    }
                }
            }
        }
    }
    
    @ViewBuilder func innerView() -> some View {
        Spacer().frame(height: 20)
        Text("STATUS")
            .font(.headline)
        Spacer().frame(height: 10)
        Text(m.setupStatusMsg)
            .font(.body)
        Spacer().frame(height: 20)
        Divider()
        VStack {
            ProgressView(value: m.mclProfilesProgress, label: { Text("Minecraft profiles") })
            ProgressView(value: m.lwjglProgress, label: { Text("LWJGL Libraries") })
            ProgressView(value: m.javaDownloadProgress, label: { Text("Java Runtime") })
        }.padding()
    }
}

extension SetupView {
    @MainActor class ViewModel: ObservableObject {
        @Published var setupStatusMsg = ""
        @Published var mclProfilesProgress = 0.0
        @Published var lwjglProgress = 0.0
        @Published var javaDownloadProgress = 0.0
        
        #if DEBUG
        init() {
            if previewMode {
                setupStatusMsg = "Downloading & decompressing libraries"
            }
        }
        #endif
        
        func downloadLibs() async {
            setupStatusMsg = "Downloading & decompressing libraries"
            
            do {
                await ["mcl_profiles.zip": \SetupView.ViewModel.mclProfilesProgress,
                       "lwjgl.zip": \SetupView.ViewModel.lwjglProgress].concurrentForEach { resource in
                    do {
                        try await downloadResource(name: resource.key, updateProgress: { fractionCompleted in
                            self[keyPath: resource.value] = fractionCompleted
                        })
                    } catch {
                        print("Failed to download resource \(resource.key).")
                        print(error.localizedDescription)
                    }
                }
                
                // TODO: Remove dependence on checksums, either have an endpoint to check for new versions or always download the latest one every launch
//                    try await downloadResource(name: "checksums.txt",
//                                               downloadProgress: progressCallback,
//                                               unzipProgress: progressCallback)
                
                // check if minecraft launcher is installed
                setupStatusMsg = "Checking minecraft launcher"
                if try Paths.global.checkMinecraftLauncherPaths() == false {
                    let alert = NSAlert()
                    alert.messageText = "Minecraft launcher problem"
                    alert.informativeText = "Minecraft launcher is not installed. Please install it and run it for the first time and try again."
                    alert.alertStyle = .critical
                    let quitBtn = alert.addButton(withTitle: "Quit")
                    quitBtn.keyEquivalent = "\r"
                    let retryBtn = alert.addButton(withTitle: "Retry")
                    retryBtn.bezelColor = .alternateSelectedControlTextColor
                    if await alert.run() == .alertFirstButtonReturn {
                        // terminate the app
                        gracefullyTerminateNoMatterWhat(sender: self)
                    }
                }
                
                // download & unzip jre
                setupStatusMsg = "Installing Java"
                let javaUrlsToDownload = [UrlValues.javaRuntime8, UrlValues.javaRuntime17]
                for (index, javaDownloadUrl) in javaUrlsToDownload.enumerated() {
                    let javaDownloadResponse = await AF.download(javaDownloadUrl, to: { _, _ in
                        return (Paths.global.dataDir.appendingPathComponent("javaRuntime.zip"), [.removePreviousFile, .createIntermediateDirectories])
                    })
                        .downloadProgress { progress in
                            self.javaDownloadProgress = (progress.fractionCompleted / Double(javaUrlsToDownload.count)) + (Double(index) / Double(javaUrlsToDownload.count))
                        }
                        .serializingDownloadedFileURL().response
                    try! FileManager.default.unzipItem(at: javaDownloadResponse.fileURL!, to: Paths.global.dataDir)
                    try! FileManager.default.removeItem(at: javaDownloadResponse.fileURL!)
                }
                // set jre exec perms
                for (_, jrePath) in Paths.global.resJre {
                    for file in try FileManager.default.contentsOfDirectory(atPath: jrePath.appendingPathComponent("Contents/Home/bin").path) {
                        var attributes = [FileAttributeKey : Any]()
                        attributes[.posixPermissions] = 0o777
                        try FileManager.default.setAttributes(attributes, ofItemAtPath: file)
                    }
                }
            } catch {
                setupStatusMsg = "Encountered an error. Please retry!"
                print("Error downloading resources")
                print(error.localizedDescription)
            }
        }
        
        static func checkLibs() -> Bool {
            print("lwjglfat.jar path: \(Paths.global.resLwjglfatJar.path)")
            let exists = FileManager.default.fileExists(atPath: Paths.global.resLwjglfatJar.path)
            print("lwjglfat.jar exists: \(exists)")
            return exists
        }
    }
}

struct SetupView_Previews: PreviewProvider {
    static var previews: some View {
        SetupView(contentViewModel: ContentView.ViewModel())
    }
}
