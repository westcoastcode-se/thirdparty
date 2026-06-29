# GLAD (https://glad.dav1d.de/)
add_library(glad STATIC src/glad.c)
target_include_directories(glad PRIVATE include)
set_property(TARGET glad PROPERTY C_STANDARD 23)
