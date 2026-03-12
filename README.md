# thirdparty

## Install pre-requisites

```bash
sudo pacman -Syu
sudo pacman -S cmake gcc 
```

## Checkout this git repository

```bash
git clone --recursive ssh://git@192.168.50.173:3022/westcoastcode-se/thirdparty.git
```

## Build and Install Libraries

```bash
./install.sh
=======
## LibPNG

```cmake
find_package(PNG 1.6 REQUIRED)
target_link_libraries(your_target PRIVATE PNG::PNG)
```
