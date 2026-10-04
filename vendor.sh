# Output Directory
OUT_DIR=$1
VENDOR_OUT_DIR=$OUT_DIR/vendor

# Copy vendor cmake file
SCRIPT_DIR=$(dirname "$0")
cp -r $SCRIPT_DIR/vendor $VENDOR_OUT_DIR

# Bash script for generating a vendor directory with the neccessary properties
git clone https://github.com/libsdl-org/SDL.git $VENDOR_OUT_DIR/SDL
cd $VENDOR_OUT_DIR/SDL; git checkout release-3.4.4; cd -;

# Clone GLFW
git clone https://github.com/glfw/glfw.git $VENDOR_OUT_DIR/glfw
cd $VENDOR_OUT_DIR/glfw; git checkout 3.4; cd -;

# Clone GLM
git clone https://github.com/g-truc/glm.git $VENDOR_OUT_DIR/glm
cd $VENDOR_OUT_DIR/glm; git checkout 1.0.3; cd -;

# Clone LibPNG
git clone https://github.com/pnggroup/libpng.git $VENDOR_OUT_DIR/libpng
cd $VENDOR_OUT_DIR/libpng; git checkout v1.6.58; cd -;

# Clone miniaudio
git clone https://github.com/mackron/miniaudio.git $VENDOR_OUT_DIR/miniaudio
cd $VENDOR_OUT_DIR/miniaudio; git checkout 0.11.25; cd -;

