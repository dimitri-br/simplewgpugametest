@echo off
rem compile vertex shaders
for %%f in (*.vert) do (
    if "%%~xf"==".vert" glslc.exe %%f -o %%f.spv
    if errorlevel 1 (
        echo Failed to compile shader %%f. Please read the error message[s] above
        exit /b 1
    )
    if "%%~xf"==".vert" echo Compiled vertex shader: %%f
)
rem compile fragment shaders
for %%f in (*.frag) do (
    if "%%~xf"==".frag" glslc.exe %%f -o %%f.spv
    if errorlevel 1 (
        echo Failed to compile shader %%f. Please read the error message[s] above
        exit /b 1
    )
    if "%%~xf"==".frag" echo Compiled fragment shader: %%f

)

exit /b 0