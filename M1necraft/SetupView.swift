//
//  SetupView.swift
//  M1necraft
//
//  Created by Raphael Tang on 2/2/22.
//

import SwiftUI
import Alamofire
import ZIPFoundation
import Path

struct SetupView: View {
    @StateObject var m = ViewModel()
    @ObservedObject var contentViewModel: ContentView.ViewModel
    @ObservedObject var resources: ResourcesViewModel
    
    var body: some View {
        if #available(macOS 12, *) {
            VStack {
                innerView()
            }
            .task(innerViewOnAppear)
            .background {
                VisualEffectView(blendingMode: .behindWindow).scaledToFill()
            }
        } else {
            VStack {
                innerView()
            }.onAppear(innerViewOnAppear)
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
            ProgressView(value: resources.mclProfilesProgress, label: { Text("Minecraft profiles") })
            ProgressView(value: resources.lwjglProgress, label: { Text("LWJGL Libraries") })
            ProgressView(value: m.javaDownloadProgress, label: { Text("Java Runtime") })
            if (m.mclDownloadProgress > 0.0) {
                ProgressView(value: m.mclDownloadProgress, label: { Text("Minecraft Launcher") })
            }
        }.padding()
    }
    
    @Sendable func innerViewOnAppear() async {
        await Runtime.preventTerminate {
            // Delete existing libs, everytime this task is run, resources should be re-downloaded.
            maybe(try Paths.dataReset())
            
            do {
                m.setupStatusMsg = "Downloading resources"
                try await resources.download()
                m.setupStatusMsg = "Downloading & decompressing libraries"
                try await m.downloadLibs()
                contentViewModel.setupStatus = .completed
            } catch {
                contentViewModel.setupStatus = .failed(AppError.setupFailure(error))
                print("Error downloading resources")
                print(error.localizedDescription)
            }
        }
    }
}

extension SetupView {
    @MainActor class ViewModel: ObservableObject {
        @Published var setupStatusMsg = ""
        @Published var javaDownloadProgress = 0.0
        @Published var mclDownloadProgress = 0.0
        
        #if DEBUG
        init() {
            if Runtime.previewMode {
                setupStatusMsg = "Downloading & decompressing libraries"
            }
        }
        #endif
        
        func downloadLibs() async throws {
            // download & unzip jre
            setupStatusMsg = "Installing Java"
            let javaUrlsToDownload = [UrlValues.javaRuntime8, UrlValues.javaRuntime17]
            for (index, javaDownloadUrl) in javaUrlsToDownload.enumerated() {
                let javaDownloadResponse = await AF.download(javaDownloadUrl, to: { _, _ in
                    return (Paths.data.join("javaRuntime.zip").url, [.removePreviousFile, .createIntermediateDirectories])
                })
                .downloadProgress { progress in
                    self.javaDownloadProgress = (progress.fractionCompleted / Double(javaUrlsToDownload.count)) + (Double(index) / Double(javaUrlsToDownload.count))
                }
                .serializingDownloadedFileURL().response
                let downloaded = P(javaDownloadResponse.fileURL!.path)!
                try! downloaded.extract(into: Paths.data)
                try! downloaded.delete()
            }
            // set jre exec perms
            for (_, jrePath) in try Paths.Resources.jre {
                try jrePath.join("Contents/Home/bin").ls().forEach { file in
                    try file.chmod(0o777)
                }
            }
            
            // install minecraft launcher
            setupStatusMsg = "Downloading Minecraft launcher"
            // install launcher
            let downloadDestinationResponse = await AF.download(UrlValues.minecraftLauncher, to: { _, _ in
                return (Paths.Minecraft.launcherDownload.url, [.removePreviousFile, .createIntermediateDirectories])
            }).downloadProgress { progress in
                self.mclDownloadProgress = progress.fractionCompleted
            }.serializingDownloadedFileURL().response
            // extract launcher
            try Util.shell("/usr/bin/hdiutil", "attach", downloadDestinationResponse.fileURL!.path.quoted)
            try Paths.Minecraft.launcherDownloadMounted
                .join("Minecraft.app")
                .copy(to: Paths.Minecraft.launcher)
            try Util.shell("/usr/bin/hdiutil", "detach", Paths.Minecraft.launcherDownloadMounted.string.quoted)
        }
        
        static func needsSetup() -> Bool {
            return [
                Paths.Minecraft.launcher.exists,
                ignoreError(try Paths.Resources.jre.allSatisfy({ $1.exists })) ?? false
            ].allSatisfy({ $0 }) == false
        }
    }
}

struct SetupView_Previews: PreviewProvider {
    static var previews: some View {
        SetupView(contentViewModel: ContentView.ViewModel(), resources: ResourcesViewModel())
    }
}
