# Configuring the official Minecraft launcher to run optimally on ARM

```
rename versions/1.16.5 to 1.16.5-arm
rename versions/1.16.5-arm/1.16.5.json to 1.16.5-arm.json
edit 1.16.5-arm.json:
     change id property to match version name 1.16.5-arm
     remove all entries in "libraries" property that contains any of the lwjgl libraries
     patch jvm arguments to something like this:
      "jvm": [
          ...
        "-Djava.library.path=${natives_directory}",
        "-Dminecraft.launcher.brand=${launcher_name}",
        "-Dminecraft.launcher.version=${launcher_version}",
        "-cp",
        "${natives_directory}/../../libraries/lwjglfat.jar:${classpath}",
        "-Dorg.lwjgl.librarypath=${natives_directory}/../../lwjglnatives/",
        "-Dfml.earlyprogresswindow=false"
      ]
restart launcher and add new profile, change version to 1.16.5-arm
also set the java path to a java version that is built for arm, e.g. Zulu 8 JDK/JRE
make sure the path ends with java, because that is the program that has to run.



to add a new launcher profile programmatically, you need to modify launcher_profiles.json.

edit launcher_profiles.json:
     under "profiles", add a new key containing a random uuid for the profile
     example:
        "d3dfdb4779992082d7d04af2797e7a7e" : {
          "created" : "2021-12-31T04:43:36.831Z",
          "icon" : "Furnace",
          "javaDir" : "/Users/raphael/Library/Application Support/minecraft/zulu-11.jdk/Contents/Home/bin/java",
          "lastUsed" : "2021-12-31T15:53:54.041Z",
          "lastVersionId" : "1.16.5-arm",
          "name" : "ARM Test",
          "type" : "custom"
        }
```
