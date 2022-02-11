//
//  Utils.swift
//  M1necraft
//
//  Created by Raphael Tang on 21/12/21.
//

import Foundation
import Alamofire
import ZIPFoundation
import AppKit
import SwiftUI
import Files

@MainActor
func downloadResource(name: String,
                      updateProgress: @escaping @MainActor (_ fractionCompleted: Double) -> Void
) async throws {
    let destinationPath = Paths.global.dataDir.appendingPathComponent(name)
    let destination: DownloadRequest.Destination = { _, _ in
        return (destinationPath, [.removePreviousFile, .createIntermediateDirectories])
    }
    let downloadDestinationResponse = await AF.download("\(UrlValues.resourceArtifactPrefix)\(name)", to: destination)
        .downloadProgress { progress in
            // fractionCompleted should not be more than 0.5 at this point
            updateProgress(progress.fractionCompleted / 2)
        }
        .serializingDownloadedFileURL().response
    // unzip
    do {
        let progress = Progress()
//        let _ = progress.publisher(for: \.fractionCompleted).sink { _ in
//            unzipProgress(progress)
//        }.store(in: &cancellables)
        let observedProgress = progress.publisher(for: \.fractionCompleted)
            .sink(receiveValue: { fractionCompleted in
                updateProgress((fractionCompleted / 2) + 0.5)
            })
        let fileManager = FileManager()
        try fileManager.unzipItem(at: downloadDestinationResponse.fileURL!, to: Paths.global.dataDir, progress: progress)
        observedProgress.cancel()
    } catch {
        print("Error extracting library")
        print(error.localizedDescription)
    }
}

extension Sequence {
    func concurrentForEach(
        _ operation: @escaping (Element) async -> Void
    ) async {
        // A task group automatically waits for all of its
        // sub-tasks to complete, while also performing those
        // tasks in parallel:
        await withTaskGroup(of: Void.self) { group in
            for element in self {
                group.addTask {
                    await operation(element)
                }
            }
        }
    }
}

extension Paths {
    func resetDataDir() throws {
        try FileManager.default.removeItem(at: dataDir)
        maybe(try FileManager.default.createDirectory(at: dataDir, withIntermediateDirectories: false))
    }
    
    func checkMinecraftLauncherPaths() throws -> Bool {
        let allChecks: [Bool] = [mclDir, mclLauncherProfiles, mclVersions].map { FileManager.default.itemExists(at: $0) }
        return allChecks.allSatisfy({ $0 })
    }
}

extension URL {
    var file: File? {
        do {
            return try File(path: self.path)
        } catch {
            print("Failed to turn URL(\(self.path)) into a File.")
            if self.hasDirectoryPath {
                print("\(self.path) is a directory. Did you mean to use .folder?")
            }
            return nil
        }
    }
    var folder: Folder? {
        do {
            return try Folder(path: self.path)
        } catch {
            print("Failed to turn URL(\(self.path)) into a Folder.")
            if !self.hasDirectoryPath {
                print("\(self.path) is a file. Did you mean to use .file?")
            }
            return nil
        }
    }
}

extension Folder {
    func copyReplace(to: Folder) throws {
        if to.containsSubfolder(named: self.name) {
            try to.subfolder(named: self.name).delete()
        }
        try self.copy(to: to)
    }
}

extension File {
    func copyReplace(to: Folder) throws {
        if to.containsFile(named: self.name) {
            try to.file(named: self.name).delete()
        }
        try self.copy(to: to)
    }
}

// MARK: - FileManager helpers
extension FileManager {
    func dirExists(atPath url: URL) -> Bool {
        var isDirectory: ObjCBool = false
        let exists = self.fileExists(atPath: url.path, isDirectory: &isDirectory)
        return exists && isDirectory.boolValue
    }
    func itemExists(at: URL) -> Bool {
        if at.hasDirectoryPath {
            return self.dirExists(atPath: at)
        } else {
            return self.fileExists(atPath: at.path)
        }
    }
}

// MARK: - NSAlert
extension NSAlert {
    /**
     Hacky workaround to runModal within an async context
     */
    @MainActor
    @discardableResult
    func run() async -> NSApplication.ModalResponse {
        await withCheckedContinuation { continuation in
            DispatchQueue.main.async { [self] in
                continuation.resume(returning: runModal())
            }
        }
    }
}

func maybe<T>(_ arg: @autoclosure () throws -> T) -> T? {
    do {
        return try arg()
    }
    catch {
        print(error.localizedDescription) // could be something else
        return nil
    }
}

// MARK: - Runtime variables
let previewMode: Bool = ProcessInfo.processInfo.environment["XCODE_RUNNING_FOR_PREVIEWS"] == "1"

private(set) var allowAppToTerminate = true // PLEASE make sure this is true i fucked this up before

func preventTerminate(_ arg: () throws -> ()) {
    do {
        allowAppToTerminate = false
        try arg()
        allowAppToTerminate = true
        NSApp.reply(toApplicationShouldTerminate: true)
    } catch {
        print(error.localizedDescription)
    }
}

func preventTerminate(_ arg: () async throws -> ()) async {
    do {
        allowAppToTerminate = false
        try await arg()
        allowAppToTerminate = true
        await NSApp.reply(toApplicationShouldTerminate: true)
    } catch {
        print(error.localizedDescription)
    }
}

func gracefullyTerminateNoMatterWhat(sender: Any?) {
    allowAppToTerminate = true
    NSApplication.shared.terminate(sender)
}

struct VisualEffectView: NSViewRepresentable {
    let blendingMode: NSVisualEffectView.BlendingMode
    
    func makeNSView(context: Context) -> NSVisualEffectView {
        let visualEffectView = NSVisualEffectView()
        visualEffectView.blendingMode = blendingMode
        visualEffectView.state = .active
        visualEffectView.appearance = NSAppearance(named: .vibrantDark)
        return visualEffectView
    }
    
    func updateNSView(_ visualEffectView: NSVisualEffectView, context: Context) {
        visualEffectView.blendingMode = blendingMode
    }
}

enum Sheet: Identifiable {
    case modInstallHelp
    case settings
    
    var id: Int {
        hashValue
    }
    
    @ViewBuilder
    func modalView(viewModel: InstallView.ViewModel) -> some View {
        switch self {
        case .modInstallHelp:  ModInstallHelpView(m: viewModel)
        case .settings:        Text("SettingsView")
        }
    }
}

enum SetupStatus {
    case loading
    case settingUp
    case completed
    case failed(Error)
}

enum MinecraftInstallState: Equatable {
    case notInstalled
    case installing(InstallationStep)
    case installed(URL)
    
    var notInstalled: Bool {
        switch self {
        case .notInstalled: return true
        default:            return false
        }
    }
    var installing: Bool {
        switch self {
        case .installing: return true
        default:          return false
        }
    }
    var installed: Bool {
        switch self {
        case .installed: return true
        default:         return false
        }
    }
}

enum InstallationStep: Equatable, CustomStringConvertible {
    case starting
    case copying(destination: String)
    case addingProfile
    case finishing
    
    var description: String {
        "(\(stepNumber)/\(stepCount)) \(message)"
    }
    
    var message: String {
        switch self {
        case .starting:
            return "Starting"
        case .copying(let destination):
            return "Copying to \(destination)"
        case .addingProfile:
            return "Adding Minecraft profile"
        case .finishing:
            return "Finishing"
        }
    }
    
    var stepNumber: Int {
        switch self {
        case .starting:      return 1
        case .copying:       return 2
        case .addingProfile: return 3
        case .finishing:     return 4
        }
    }
    
    var stepCount: Int { 3 }
}

func configure<Subject>(_ subject: Subject, configuration: (inout Subject) -> Void) -> Subject {
    var copy = subject
    configuration(&copy)
    return copy
}

/// You probably want ProgressView unless you need more of NSProgressIndicator's API, which this exposes.
struct ProgressIndicator: NSViewRepresentable {
    typealias NSViewType = NSProgressIndicator
    
    let minValue: Double
    let maxValue: Double
    let doubleValue: Double
    let controlSize: NSControl.ControlSize
    let isIndeterminate: Bool
    let style: NSProgressIndicator.Style
    
    func makeNSView(context: Context) -> NSViewType {
        NSProgressIndicator()
    }

    func updateNSView(_ nsView: NSViewType, context: Context) {
        nsView.minValue = minValue
        nsView.maxValue = maxValue
        nsView.doubleValue = doubleValue
        nsView.controlSize = controlSize
        nsView.isIndeterminate = isIndeterminate
        nsView.style = style
    }
}

// MARK: - Version utilities
struct MinecraftVersionParsed {
    let major: Int
    let minor: Int?
    
    init?(string: String) {
        let components = string.split(separator: ".")
        if components.isEmpty { return nil }
        
        major = Int(components[1]) ?? 0
        if components.indices.contains(2) {
            minor = Int(components[2]) ?? 0
        } else {
            minor = nil
        }
    }
}

extension M1necraftVersion {
    var isInstalledAt: URL? {
        let path = Paths.global.mclVersions.appendingPathComponent("\(name)-arm")
        return FileManager.default.dirExists(atPath: path) ? path : nil
    }
    var javaVersion: Int {
        MinecraftVersionParsed(string: name)!.major >= 17 ? 17 : 8
    }
}
