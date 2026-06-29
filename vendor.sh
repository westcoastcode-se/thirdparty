# Copy vendor cmake file
SCRIPT_DIR=$(dirname "$0")
cp -r $SCRIPT_DIR/vendor vendor

# Bash script for generating a vendor directory with the neccessary properties
git clone https://github.com/libsdl-org/SDL.git vendor/SDL
cd vendor/SDL; git checkout release-3.4.4; cd -;

# Clone GLFW
git clone https://github.com/glfw/glfw.git vendor/glfw
cd vendor/glfw; git checkout 3.4; cd -;

# Clone GLM
git clone https://github.com/g-truc/glm.git vendor/glm
cd vendor/glm; git checkout 1.0.3; cd -;

# Clone LibPNG
git clone https://github.com/pnggroup/libpng.git vendor/libpng
cd vendor/libpng; git checkout v1.6.58; cd -;
