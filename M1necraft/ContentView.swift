//
//  ContentView.swift
//  M1necraft
//
//  Created by Raphael Tang on 18/12/21.
//

import SwiftUI
import OctoKit

struct ContentView: View {
    @EnvironmentObject var m: ContentView.ViewModel
    @EnvironmentObject var updater: UpdaterViewModel
    @StateObject var resources = ResourcesViewModel()
    
    var body: some View {
        Group {
            switch m.setupStatus {
            case .loading:
                VStack {
                    Text("Loading...")
                }
            case .settingUp:
                SetupView(resources: resources)
            case .completed:
                InstallView()
            case .failed(let error):
                VStack {
                    Text("We've encountered an error during the setup process.")
                    ErrorView(error).padding()
                    Button("Retry") {
                        m.setupStatus = .settingUp
                    }
                }
                .padding(.all, 40)
            }
        }.toolbar {
            Button {
                print("info button pressed")
            } label: {
                Image(systemName: "info.circle")
            }
            Button {
                m.activeSheet = .settings
            } label: {
                Image(systemName: "gearshape")
            }
        }.onAppear {
            if SetupView.ViewModel.needsSetup() {
                m.setupStatus = .settingUp
            } else {
                m.setupStatus = .completed
                
                do {
                    if try await resources.checkForUpdate() {
                        try await resources.download()
                    }
                } catch {
                    m.setupStatus = .failed(AppError.setupFailure(error))
                }
            }
        }.sheet(item: $m.activeSheet) {
            $0.modalView()
        }
    }
}

extension ContentView {
    @MainActor class ViewModel: ObservableObject {
        @Published var setupStatus: SetupStatus = .loading
        @Published var activeSheet: Sheet?
    }
}

struct ContentView_Previews: PreviewProvider {
    static var previews: some View {
        ContentView()
    }
}
