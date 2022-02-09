//
//  GameVersionListView.swift
//  M1necraft
//
//  Created by Raphael Tang on 7/2/22.
//

import SwiftUI

struct GameVersionListView: View {
    @ObservedObject var m: InstallView.ViewModel
    
    var body: some View {
        List(m.versions, selection: $m.selectedMinecraftVersionID) {
            GameVersionListItem(m: m, version: $0, selected: m.selectedMinecraftVersionID == $0.id)
        }.onAppear {
            m.refreshVersions() // set all version states
        }
    }
}

struct GameVersionListView_Previews: PreviewProvider {
    static var previews: some View {
        GameVersionListView(m: InstallView.ViewModel())
    }
}
