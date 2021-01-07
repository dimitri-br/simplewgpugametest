@echo off
for %%f in (*.vert) do (
    if "%%~xf"==".vert" glslc.exe %%f -o %%f.spv
    if errorlevel 1 (
        echo Failed to compile shader %%f. Please read the error message[s] above
        exit /b 1
    )
    if "%%~xf"==".vert" echo Compiled vertex shader: %%f
)

for %%f in (*.frag) do (
    if "%%~xf"==".frag" glslc.exe %%f -o %%f.spv
    if errorlevel 1 (
        echo Failed to compile shader %%f. Please read the error message[s] above
        exit /b 1
    )
    if "%%~xf"==".frag" echo Compiled fragment shader: %%f

)

pause