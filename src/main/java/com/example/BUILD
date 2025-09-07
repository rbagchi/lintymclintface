java_library(
    name = "greeter",
    srcs = ["Greeter.java"],
    visibility = ["//src/main/java/com/example:__pkg__"],
)

java_binary(
    name = "main",
    srcs = ["Main.java"],
    main_class = "com.example.Main",
    deps = [":greeter"],
)
