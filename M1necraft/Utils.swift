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
import Path
import OctoKit

class Util {
    static func resetData() throws {
        try Paths.dataReset()
        try Paths.Minecraft.lwjglnatives.delete()
        try Paths.Minecraft.lwjglfatJar.delete()
        try Paths.Minecraft.jre.forEach { folder in
            try folder.value.delete()
        }

        // delete all installed versions
        try M1necraftVersion.all.forEach { version in
            try version.isInstalledAt?.delete()
        }

        let jsonEncoder = JSONEncoder()
        let jsonDecoder = JSONDecoder()
        
        if Paths.Minecraft.launcherProfiles.exists {
            var launcherProfiles = try jsonDecoder.decode(MinecraftLauncherProfiles.self,
                                                          from: Data(contentsOf: Paths.Minecraft.launcherProfiles))
            launcherProfiles.profiles.forEach { profileItem in
                if profileItem.key.hasPrefix("m1necraft") {
                    launcherProfiles.profiles.removeValue(forKey: profileItem.key)
                }
            }

            try String(data: jsonEncoder.encode(launcherProfiles), encoding: .utf8)?.write(to: Paths.Minecraft.launcherProfiles.url, atomically: false, encoding: .utf8)
        }
    }
    
    @discardableResult
    static func shell(_ command: String...) throws -> (String, Int32) {
        let task = Process()
        let pipe = Pipe()
        
        task.standardOutput = pipe
        task.standardError = pipe
        task.arguments = ["-c", command.joined(separator: " ")]
        task.executableURL = URL(fileURLWithPath: "/bin/sh")
        
        try task.run()
        
        let data = pipe.fileHandleForReading.readDataToEndOfFile()
        let output = String(data: data, encoding: .utf8) ?? ""
        task.waitUntilExit()
        
        return (output, task.terminationStatus)
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

extension Paths.Minecraft {
    static func requiredExists() -> Bool {
        return [
            launcher.exists,
            workDir.exists,
            launcherProfiles.exists,
            versions.exists,
            libraries.exists,
            runtime.exists,
        ].allSatisfy({ $0 })
    }
}

extension P {
    func extract(into: Self,
                 skipCRC32: Bool = false,
                 progress: Progress? = nil,
                 preferredEncoding: String.Encoding? = nil)
    throws {
        try FileManager.default.unzipItem(at: url, to: into.url, skipCRC32: skipCRC32, progress: progress, preferredEncoding: preferredEncoding)
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

@discardableResult func maybe<T>(_ arg: @autoclosure () throws -> T) -> T? {
    do {
        return try arg()
    }
    catch {
        print(error.localizedDescription) // could be something else
        return nil
    }
}

@discardableResult func ignoreError<T>(_ arg: @autoclosure () throws -> T) -> T? {
    do {
        return try arg()
    }
    catch {
        return nil
    }
}

// MARK: - Runtime variables
class Runtime {
    static var previewMode: Bool {
        get {
            return ProcessInfo.processInfo.environment["XCODE_RUNNING_FOR_PREVIEWS"] == "1"
        }
    }
    
    private(set) static var allowAppToTerminate = true // PLEASE make sure this is true i fucked this up before
    
    static func preventTerminate(_ arg: () throws -> ()) {
        do {
            allowAppToTerminate = false
            try arg()
            allowAppToTerminate = true
            NSApp.reply(toApplicationShouldTerminate: true)
        } catch {
            print(error.localizedDescription)
        }
    }

    static func preventTerminate(_ arg: () async throws -> ()) async {
        do {
            allowAppToTerminate = false
            try await arg()
            allowAppToTerminate = true
            await NSApp.reply(toApplicationShouldTerminate: true)
        } catch {
            print(error.localizedDescription)
        }
    }

    static func forceTerminate(_ sender: Any?) {
        allowAppToTerminate = true
        NSApp.reply(toApplicationShouldTerminate: true)
        NSApp.windows.forEach { window in
            window.close()
        }
        NSApp.terminate(sender)
    }
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
    func modalView() -> some View {
        switch self {
        case .modInstallHelp:  ModInstallHelpView()
        case .settings:        SettingsView(showCloseButton: true)
        }
    }
}

enum SetupStatus {
    case loading
    case settingUp
    case completed
    case failed(AppError)
}

enum MinecraftInstallState: Equatable {
    case notInstalled
    case installing(InstallationStep)
    case installed(P)
    
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
    case copying
    case addingProfile
    case finishing
    
    var description: String {
        "(\(stepNumber)/\(stepCount)) \(message)"
    }
    
    var message: String {
        switch self {
        case .starting:
            return "Starting"
        case .copying:
            return "Copying files"
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
    var isInstalledAt: P? {
        let installedPath = Paths.Minecraft.versions/"\(name)-arm"
        return installedPath.exists ? installedPath : nil
    }
    
    var javaVersion: Int {
        MinecraftVersionParsed(string: name)!.major >= 17 ? 17 : 8
    }
    
    static let all: [Self] = {
        var allVersions: [Self] = Paths.Resources.mclProfiles.ls().directories.map { (path: P) in
            return M1necraftVersion(name: String(path.url.lastPathComponent.dropLast(4)))
        }
        allVersions = allVersions.sorted { $0 > $1 }
        return allVersions
    }()
}

class Builder<T> {
    private var object: T
    
    init(_ obj: T) {
        object = obj
    }
    
    func with<V>(_ property: WritableKeyPath<T, V>, _ value: V) -> Self {
        object[keyPath: property] = value
        return self
    }
    
    func build() -> T {
        object
    }
}

// MARK: - Minecraft Launcher
struct MinecraftLauncher {
    static func run() {
        NSWorkspace.shared.openApplication(at: Paths.Minecraft.launcher.url, configuration: Builder(NSWorkspace.OpenConfiguration())
            .with(\.activates, true)
            .with(\.arguments, ["--workDir", Paths.Minecraft.workDir.string, "--lockDir", Paths.data.string])
            .build())
    }
    
    static func createLauncherFiles() throws {
        try Paths.Minecraft.workDir.mkdir()
        try String(data: JSONEncoder().encode(MinecraftLauncherProfiles.default()), encoding: .utf8)?.write(to: Paths.Minecraft.launcherProfiles)
        try Paths.Minecraft.versions.mkdir()
        try Paths.Minecraft.libraries.mkdir()
        try Paths.Minecraft.runtime.mkdir()
    }
}

extension String {
    var quoted: Self {
        return "'\(self)'"
    }
}

@MainActor class ResourcesViewModel: ObservableObject {
    @AppStorage("resourcesPublishedAt") var resourcesPublishedAt: TimeInterval = 0
    @Published var mclProfilesProgress = 0.0
    @Published var lwjglProgress = 0.0
    
    func checkForUpdate() async throws -> Bool {
        var hasNewResources = false

        let release: Release = try await withCheckedThrowingContinuation { continuation in
            Octokit().release(owner: "raphtlw", repository: "m1necraft", tag: "resources") { response in
                switch response {
                case .success(let value):
                    continuation.resume(returning: value)
                case .failure(let error):
                    continuation.resume(throwing: error)
                }
            }
        }

        if resourcesPublishedAt > 0 {
            if release.publishedAt!.timeIntervalSince1970 > resourcesPublishedAt {
                hasNewResources = true
            }
        }
        resourcesPublishedAt = release.publishedAt!.timeIntervalSince1970

        return hasNewResources
    }
    
    @MainActor
    func downloadResource(name: String, updateProgress: @escaping @MainActor (_ fractionCompleted: Double) -> Void
    ) async throws {
        let destinationPath = Paths.data.url.appendingPathComponent(name)
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
            let observedProgress = progress.publisher(for: \.fractionCompleted)
                .sink(receiveValue: { fractionCompleted in
                    updateProgress((fractionCompleted / 2) + 0.5)
                })
            try P(downloadDestinationResponse.fileURL!.path)?.extract(into: Paths.data, progress: progress)
            observedProgress.cancel()
        } catch {
            print("Error extracting library")
            print(error.localizedDescription)
        }
    }
    
    func download() async throws {
        try delete()
        
        await ["mcl_profiles.zip": \ResourcesViewModel.mclProfilesProgress,
               "lwjgl.zip": \ResourcesViewModel.lwjglProgress].concurrentForEach { [self] resource in
            do {
                try await downloadResource(name: resource.key, updateProgress: { fractionCompleted in
                    self[keyPath: resource.value] = fractionCompleted
                })
            } catch {
                print("Failed to download resource \(resource.key).")
                print(error.localizedDescription)
            }
        }
    }
    
    func delete() throws {
        try Paths.Resources.mclProfiles.delete()
        try Paths.Resources.lwjgl.delete()
    }
}

extension View {
    @ViewBuilder func onAppear(_ action: @MainActor @Sendable @escaping () async -> Void) -> some View {
        if #available(macOS 12.0, *) {
            task(action)
        } else {
            onAppear {
                Task {
                    await action()
                }
            }
        }
    }
}
