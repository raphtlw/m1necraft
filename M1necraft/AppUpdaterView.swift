//
//  AppUpdaterView.swift
//  M1necraft
//
//  Created by Raphael Tang on 6/5/22.
//

import SwiftUI
import Sparkle

final class UpdaterViewModel: ObservableObject {
    private let updaterController: SPUStandardUpdaterController
    
    @Published var canCheckForUpdates = false
    
    init() {
        updaterController = SPUStandardUpdaterController(startingUpdater: true, updaterDelegate: nil, userDriverDelegate: nil)
        updaterController.updater.publisher(for: \.canCheckForUpdates).assign(to: &$canCheckForUpdates)
    }
    
    func checkForUpdates() {
        updaterController.checkForUpdates(nil)
    }
}

struct AppUpdaterView: View {
    @ObservedObject var updater: UpdaterViewModel
    
    var body: some View {
        Button("Check for Updates...", action: updater.checkForUpdates)
            .disabled(!updater.canCheckForUpdates)
    }
}

struct AppUpdaterView_Previews: PreviewProvider {
    static var previews: some View {
        AppUpdaterView(updater: UpdaterViewModel())
    }
}
