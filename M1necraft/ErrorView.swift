//
//  ErrorView.swift
//  M1necraft
//
//  Created by Raphael Tang on 2/5/22.
//

import SwiftUI

struct ErrorView: View {
    var error: AppError
    @State var messageText: String = ""
    
    init(_ error: AppError) {
        self.error = error
        self.messageText = error.localizedDescription
    }
    
    var body: some View {
        TextEditor(text: .constant(error.localizedDescription))
            .focusable(true)
            .font(Font.system(size: 12, design: .monospaced))
    }
}

struct ErrorView_Previews: PreviewProvider {
    static var previews: some View {
        ErrorView(AppError.loremIpsum)
    }
}
