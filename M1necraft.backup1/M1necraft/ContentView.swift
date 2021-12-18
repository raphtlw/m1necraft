//
//  ContentView.swift
//  M1necraft
//
//  Created by Raphael Tang on 18/12/21.
//

import SwiftUI

struct ContentView: View {
    @AppStorage("minecraftUsername") private var username: String = ""
    @AppStorage("minecraftPassword") private var password: String = ""
    
    var body: some View {
        VStack {
            VStack {
                Text("m1necraft")
                    .padding()
                    .font(.title.weight(.bold))
                Text("Minecraft for Apple Silicon.")
            }
            .padding()
            TextField(
                "Username",
                text: $username
            )
            SecureField(
                "Password",
                text: $password
            )
            Button("Continue as Guest", action: {
                
            })
                .buttonStyle(.borderless)
                .padding()
            HStack {
                Button("Login", action: {
                    print("Login pressed")
                })
                    .keyboardShortcut(.defaultAction)
            }
            .frame(maxWidth: .infinity, alignment: .trailing)
        }
        .padding()
    }
}

struct ContentView_Previews: PreviewProvider {
    static var previews: some View {
        ContentView()
    }
}
