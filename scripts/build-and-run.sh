#!/usr/bin/env bash

xcodebuild -project M1necraft.xcodeproj -scheme M1necraft build

x=$( xcodebuild -showBuildSettings -project M1necraft.xcodeproj | grep ' BUILD_DIR =' | sed -e 's/.*= *//' )

DYLD_FRAMEWORK_PATH=$x/Debug DYLD_LIBRARY_PATH=$x/Debug $x/Debug/M1necraft.app/Contents/MacOS/M1necraft
