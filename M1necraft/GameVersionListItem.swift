//
//  GameVersionListItem.swift
//  M1necraft
//
//  Created by Raphael Tang on 7/2/22.
//

import SwiftUI

struct GameVersionListItem: View {
    @ObservedObject var m: InstallView.ViewModel
    @State var version: M1necraftVersion
    @State var showModInstallHelpModal = false
    let selected: Bool
    
    var body: some View {
        HStack {
            Image("MinecraftIcon")
                .resizable()
                .scaledToFit()
                .frame(width: 25)
            Spacer().frame(width: 15)
            
            VStack(alignment: .leading) {
                Text(verbatim: version.name)
                
                if case let .installed(path) = version.installState {
                    Text(verbatim: path.path)
                        .font(.caption)
                        .foregroundColor(.secondary)
                } else {
                    Text(verbatim: "")
                        .font(.caption)
                }
            }
            
            Spacer()
            
            switch version.installState {
            case .installed:
                Menu {
                    Button("Install Fabric", action: {
                        m.activeSheet = .modInstallHelp
                    })
                    Button("Install Forge", action: {
                        m.activeSheet = .modInstallHelp
                    })
                } label: {
                    Image(systemName: "ellipsis.circle")
                }
                .menuStyle(.borderlessButton)
                .menuIndicator(.hidden)
                .fixedSize()
                .padding(.trailing, 5)
                Button("OPEN", action: {
                    preventTerminate {
                        guard let appUrl = NSWorkspace.shared.urlForApplication(withBundleIdentifier: Strings.minecraftLauncherBundleID) else { return }
                        NSWorkspace.shared.openApplication(at: appUrl, configuration: NSWorkspace.OpenConfiguration(), completionHandler: nil)
                    }
                })
                .buttonStyle(AppStoreButtonStyle(primary: true, highlighted: selected))
            case .notInstalled:
                Button("INSTALL", action: {
                    preventTerminate {
                        m.installVersion(version: version, updateState: { installationStep in
                            version.installState = .installing(installationStep)
                        })
                    }
                })
                .buttonStyle(AppStoreButtonStyle(primary: false, highlighted: selected))
            case let .installing(installationStep):
                installationStepView(for: installationStep)
            }
        }
    }
    
    @ViewBuilder
    func installationStepView(for installationStep: InstallationStep) -> some View {
        HStack {
            switch installationStep {
            case .starting, .copying, .addingProfile, .finishing:
                ProgressView().scaleEffect(0.5)
            }
            
            Text("Step \(installationStep.stepNumber) of \(installationStep.stepCount): \(installationStep.message)")
                .font(.footnote)
            Button(action: {}) {
                Label("Cancel", systemImage: "xmark.circle.fill")
                    .labelStyle(IconOnlyLabelStyle())
            }
            .buttonStyle(PlainButtonStyle())
            .foregroundColor(selected ? .white : .secondary)
            .help("Stop installation")
        }
    }
}

struct GameVersionListItem_Previews: PreviewProvider {
    static var previews: some View {
        GameVersionListItem(m: InstallView.ViewModel(), version: M1necraftVersion(name: "1.16.5"), selected: false)
    }
}
