//
//  ContentView.swift
//  M1necraft
//
//  Created by Raphael Tang on 18/12/21.
//

import SwiftUI

struct ContentView: View {
    @ObservedObject var m: ContentView.ViewModel
    
    var body: some View {
        Group {
            switch m.setupStatus {
            case .loading:
                VStack {
                    Text("Loading...")
                }
            case .settingUp:
                SetupView(contentViewModel: m)
            case .completed:
                InstallView(contentViewModel: m)
            case .failed(let error):
                VStack {
                    Text(error.localizedDescription)
                }
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
            if SetupView.ViewModel.checkLibs() {
                m.setupStatus = .completed
            } else {
                m.setupStatus = .settingUp
            }
        }.sheet(item: $m.activeSheet) {
            $0.modalView(viewModel: m)
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
        ContentView(m: ContentView.ViewModel())
    }
}
