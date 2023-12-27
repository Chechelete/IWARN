@echo off
set "DownloadURL=https://raw.githubusercontent.com/Chechelete/IWARN/master/IWARN_WINDOWS/IWARN.exe"
set "TempFile=%TEMP%\IWARN.exe"
set "InstallDir=%ProgramFiles(x86)%\IWARN"

echo Downloading IWARN.exe...
powershell -Command "& { Invoke-WebRequest -Uri '%DownloadURL%' -OutFile '%TempFile%' }"

echo Installing IWARN.exe to %InstallDir%...
if not exist "%InstallDir%" mkdir "%InstallDir%"
copy /Y "%TempFile%" "%InstallDir%"

if exist "%InstallDir%\IWARN.exe" (
    echo Adding IWARN directory to PATH...
    setx PATH "%PATH%;%InstallDir%" /M
    echo Installation completed.
) else (
    echo Installation failed. Please check for errors.
)
pause