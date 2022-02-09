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
                InstallView()
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
                print("settings button pressed")
            } label: {
                Image(systemName: "gearshape")
            }
        }.onAppear {
            if SetupView.ViewModel.checkLibs() {
                m.setupStatus = .completed
            } else {
                m.setupStatus = .settingUp
            }
        }
    }
}

extension ContentView {
    @MainActor class ViewModel: ObservableObject {
        @Published var setupStatus: SetupStatus = .loading
    }
}

struct ContentView_Previews: PreviewProvider {
    static var previews: some View {
        ContentView(m: ContentView.ViewModel())
    }
}
