//
//  Path.swift
//  M1necraft
//
//  Created by Raphael Tang on 2/5/22.
//

import Path

/**
 A `Path` represents an absolute path on a filesystem.

 All functions on `Path` are chainable and short to facilitate doing sequences
 of file operations in a concise manner.

 `Path` supports `Codable`, and can be configured to
 [encode paths *relatively*](https://github.com/mxcl/Path.swift/#codable).

 Sorting a `Sequence` of paths will return the locale-aware sort order, which
 will give you the same order as Finder.

 Converting from a `String` is a common first step, here are the recommended
 ways to do that:

     let p1 = Path.root/pathString
     let p2 = Path.root/url.path
     let p3 = Path.cwd/relativePathString
     let p4 = Path(userInput) ?? Path.cwd/userInput

 If you are constructing paths from static-strings we provide support for
 dynamic members:

     let p1 = Path.root.usr.bin.ls  // => /usr/bin/ls

 However we only provide this support off of the static members like `root` due
 to the anti-pattern where Path.swift suddenly feels like Javascript otherwise.

 - Note: A `Path` does not necessarily represent an actual filesystem entry.
 - SeeAlso: `Pathish` for most methods you will use on `Path` instances.
 */
typealias P = Path
