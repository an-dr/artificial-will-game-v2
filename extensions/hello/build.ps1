#!/usr/bin/env pwsh
# Builds the hello extension into a WASM component, builds the bones
# engine (vendored as a submodule at repo root), and assembles a runnable
# dist/ next to it. Run with: pwsh build.ps1
$ErrorActionPreference = "Stop"

rustup target add wasm32-wasip2
if ($LASTEXITCODE -ne 0) { exit $LASTEXITCODE }

cargo build --target wasm32-wasip2 --release
if ($LASTEXITCODE -ne 0) { exit $LASTEXITCODE }

Write-Host "Built: target/wasm32-wasip2/release/hello.wasm"

# core/platform builds SDL3 from source via cmake. Any working C compiler
# does (MSVC, clang, clang-cl, gcc) -- prefer whatever this machine already
# has set up over assuming MSVC specifically.
function Initialize-NativeBuildEnvironment {
    if ((Get-Command ninja -ErrorAction SilentlyContinue) -and -not $env:CMAKE_GENERATOR) {
        $env:CMAKE_GENERATOR = "Ninja"
    }

    $compilers = "cl", "clang-cl", "clang", "gcc", "cc"
    if ($compilers | Where-Object { Get-Command $_ -ErrorAction SilentlyContinue }) {
        return # something is already on PATH -- respect it, don't override
    }
    if (-not $IsWindows) { return } # the MSVC fallback below is Windows-only

    $vcvarsall = @(
        "${env:ProgramFiles}\Microsoft Visual Studio\*\*\VC\Auxiliary\Build\vcvarsall.bat",
        "${env:ProgramFiles(x86)}\Microsoft Visual Studio\*\*\VC\Auxiliary\Build\vcvarsall.bat"
    ) | ForEach-Object { Get-Item -Path $_ -ErrorAction SilentlyContinue } | Select-Object -First 1
    if (-not $vcvarsall) {
        Write-Host "Note: no C compiler found and no vcvarsall.bat located; continuing as-is."
        return
    }

    $arch = if ($env:PROCESSOR_ARCHITECTURE -eq "ARM64") { "arm64" } else { "x64" }
    Write-Host "==> No compiler on PATH; loading MSVC ($($vcvarsall.FullName) $arch)..."
    cmd /c "`"$($vcvarsall.FullName)`" $arch >nul 2>&1 && set" | ForEach-Object {
        if ($_ -match '^([^=]+)=(.*)$') {
            Set-Item -Path "env:$($matches[1])" -Value $matches[2]
        }
    }
}

$bonesRoot = Resolve-Path "$PSScriptRoot/../../bones"
$exeName = if ($IsWindows) { "bones.exe" } else { "bones" }
$dist = "$PSScriptRoot/dist"

Write-Host "==> Building bones..."
Push-Location $bonesRoot
try {
    Initialize-NativeBuildEnvironment
    cargo build -p app --release
    if ($LASTEXITCODE -ne 0) { exit $LASTEXITCODE }
} finally {
    Pop-Location
}

Remove-Item -Recurse -Force $dist -ErrorAction SilentlyContinue
New-Item -ItemType Directory -Path "$dist/extensions" -Force | Out-Null
Copy-Item "$bonesRoot/target/release/$exeName" "$dist/$exeName"
Copy-Item "$PSScriptRoot/target/wasm32-wasip2/release/hello.wasm" "$dist/extensions/hello.wasm"

Write-Host ""
Write-Host "Packaged: $dist/$exeName (extensions/hello.wasm alongside it)"
