//
//  Main.swift
//  M1necraft
//
//  Created by Raphael Tang on 6/2/22.
//

import Foundation
import AppKit

@main
struct Main {
    static func main() {
        if #available(OSX 11.0, *) {
            M1necraftApp.main()
        } else {
            M1necraftAppCompat.main()
        }
    }
}
