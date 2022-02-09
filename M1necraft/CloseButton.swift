//
//  CloseButton.swift
//  M1necraft
//
//  Created by Raphael Tang on 8/2/22.
//

import SwiftUI

struct CloseButton: View {
    let action: () -> Void
    
    var body: some View {
        Button(action: action, label: {
            Image(systemName: "xmark.circle.fill")
                .resizable()
                .scaledToFit()
                .frame(width: 16)
        })
        .buttonStyle(.plain)
        .accessibilityLabel("Close")
        .accessibilityHint("Close this screen")
    }
}

struct CloseButton_Previews: PreviewProvider {
    static var previews: some View {
        CloseButton(action: {})
    }
}
