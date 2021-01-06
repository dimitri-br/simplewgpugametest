@echo off
for %%f in (*.vert) do (
    if "%%~xf"==".vert" C:\VulkanSDK\1.2.148.1\Bin32\glslc.exe %%f -o %%f.spv
    if "%%~xf"==".vert" echo Compiled vertex shader: %%f
)

for %%f in (*.frag) do (
    if "%%~xf"==".frag" C:\VulkanSDK\1.2.148.1\Bin32\glslc.exe %%f -o %%f.spv
    if "%%~xf"==".frag" echo Compiled fragment shader: %%f
)