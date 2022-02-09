////
////  LoginView.swift
////  M1necraft
////
////  Created by Raphael Tang on 19/12/21.
////
//
//import SwiftUI
//import KeychainSwift
//
//struct LoginView: View {
//    @ObservedObject var m: ContentView.ViewModel
//    @AppStorage("signedIn") private var signedIn = false
//    
//    init(_ m: ContentView.ViewModel) {
//        self.m = m
//    }
//    
//    @State private var showUsernameSheet: Bool = false
//    
//    func completeSignIn() {
//        signedIn = true
//    }
//    
//    var body: some View {
//        VStack {
//            VStack {
//                Text("m1necraft")
//                    .padding()
//                    .font(.title.weight(.bold))
//                Text("Minecraft for Apple Silicon.")
//            }
//            .padding()
//            TextField(
//                "Username",
//                text: $m.minecraftUsername
//            )
//            SecureField(
//                "Password",
//                text: $m.minecraftPassword
//            )
//            Button("Continue as Guest", action: {
//                showUsernameSheet = true
//            })
//                .buttonStyle(.borderless)
//                .padding()
//            HStack {
//                Button("Login", action: {
//                    let keychain = KeychainSwift()
//                    keychain.set(m.minecraftPassword, forKey: "m1necraft_password")
//                    completeSignIn()
//                })
//                    .keyboardShortcut(.defaultAction)
//            }
//            .frame(maxWidth: .infinity, alignment: .trailing)
//        }
//        .padding()
//        
//        .sheet(isPresented: $showUsernameSheet) {
//            VStack {
//                Text("Please enter a name to use for Minecraft")
//                Spacer().frame(height: 20)
//                TextField(
//                    "Username",
//                    text: $m.minecraftUsername
//                )
//                Button("Done", action: {
//                    showUsernameSheet = false
//                    completeSignIn()
//                })
//                    .keyboardShortcut(.defaultAction)
//            }
//            .padding()
//        }
//    }
//}
//
//struct LoginView_Previews: PreviewProvider {
//    static var previews: some View {
//        LoginView(ContentView.ViewModel())
//    }
//}
