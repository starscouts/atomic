const fs = require('fs');
const cp = require('child_process');
const plist = require('plist');

if (fs.existsSync("./work")) fs.rmSync("./work", { recursive: true });
fs.mkdirSync("./work");
fs.writeFileSync("./work/package.json", "{}");

console.log("# Downloading Electron for macOS ARM64");
cp.execSync("npm install electron@latest --platform=darwin --arch=arm64", { cwd: "./work", stdio: "inherit" });
fs.renameSync("./work/node_modules/electron/dist", "./work/electron-darwin-arm64");
fs.rmSync("./work/node_modules", { recursive: true });

console.log("# Downloading Electron for macOS x64");
cp.execSync("npm install electron@latest --platform=darwin --arch=x64", { cwd: "./work", stdio: "inherit" });
fs.renameSync("./work/node_modules/electron/dist", "./work/electron-darwin-x64");
fs.rmSync("./work/node_modules", { recursive: true });

console.log("# Downloading Electron for Windows x64");
cp.execSync("npm install electron@latest --platform=win32 --arch=x64", { cwd: "./work", stdio: "inherit" });
fs.renameSync("./work/node_modules/electron/dist", "./work/electron-win32-x64");
fs.rmSync("./work/node_modules", { recursive: true });

let version = fs.readFileSync("./work/electron-darwin-arm64/version").toString().trim();
console.log("# Electron version: " + version);

console.log("# Preparing Electron");
fs.mkdirSync("./work/root");
fs.mkdirSync("./work/root/arm");
fs.mkdirSync("./work/root/x64");

fs.renameSync("./work/electron-darwin-x64/Electron.app", "./work/root/x64/AtomicRuntime.framework");
fs.renameSync("./work/electron-darwin-arm64/Electron.app", "./work/root/arm/AtomicRuntime.framework");
fs.renameSync("./work/root/x64/AtomicRuntime.framework/Contents/MacOS/Electron", "./work/root/x64/AtomicRuntime.framework/Contents/MacOS/Atomic");
fs.renameSync("./work/root/arm/AtomicRuntime.framework/Contents/MacOS/Electron", "./work/root/arm/AtomicRuntime.framework/Contents/MacOS/Atomic");

fs.renameSync("./work/electron-win32-x64", "./work/root/x64/AtomicRuntime");
fs.renameSync("./work/root/x64/AtomicRuntime/electron.exe", "./work/root/x64/AtomicRuntime/atomic.exe");

fs.rmSync("./work/root/arm/AtomicRuntime.framework/Contents/Info.plist");
fs.rmSync("./work/root/arm/AtomicRuntime.framework/Contents/PkgInfo");
fs.rmSync("./work/root/x64/AtomicRuntime.framework/Contents/Info.plist");
fs.rmSync("./work/root/x64/AtomicRuntime.framework/Contents/PkgInfo");

console.log("# Preparing SDK");
fs.mkdirSync("./work/root/sdk");
fs.mkdirSync("./work/root/sdk/Atomic");
fs.mkdirSync("./work/root/sdk/Atomic/SDK");

for (let file of fs.readdirSync("../converter/").filter(i => i.startsWith("convert_"))) {
    fs.copyFileSync("../converter/" + file, "./work/root/sdk/Atomic/SDK/" + file);
}

console.log("# Preparing standalone launcher");
fs.mkdirSync("./work/root/launcher");
fs.mkdirSync("./work/root/launcher/Atomic");
fs.mkdirSync("./work/root/launcher/Atomic/Launcher");

for (let file of fs.readdirSync("../converter/").filter(i => i.startsWith("launcher-"))) {
    fs.copyFileSync("../converter/" + file, "./work/root/launcher/Atomic/Launcher/" + file);
}

console.log("# Generating installer for macOS x64");

let data = plist['parse'](fs.readFileSync("./Atomic-x64.pkgproj").toString());
data["PACKAGES"].filter(i => i["PACKAGE_SETTINGS"]["NAME"] === "Atomic Runtime")[0]["PACKAGE_SETTINGS"]["VERSION"] = version;
fs.writeFileSync("./Atomic-x64-work.pkgproj", plist['build'](data)
    .replaceAll("  ", "\t")
    .replace(/^\t(?=<dict>|<\/dict>)|^\t(?=\t(?=[<\t]))/gm, "")
    .replaceAll("<string/>", "<string></string>")
    .replaceAll("<data/>", "<data></data>")
);

cp.execSync("packagesbuild ./Atomic-x64-work.pkgproj", { stdio: "inherit" });

console.log("# Generating installer for macOS ARM");

cp.execSync("packagesbuild ./Atomic-ARM-work.pkgproj", { stdio: "inherit" });

data = plist['parse'](fs.readFileSync("./Atomic-ARM.pkgproj").toString());
data["PACKAGES"].filter(i => i["PACKAGE_SETTINGS"]["NAME"] === "Atomic Runtime")[0]["PACKAGE_SETTINGS"]["VERSION"] = version;
fs.writeFileSync("./Atomic-ARM-work.pkgproj", plist['build'](data)
    .replaceAll("  ", "\t")
    .replace(/^\t(?=<dict>|<\/dict>)|^\t(?=\t(?=[<\t]))/gm, "")
    .replaceAll("<string/>", "<string></string>")
    .replaceAll("<data/>", "<data></data>")
);

console.log("# Generating installer for Windows x64");
fs.writeFileSync("./windows-work.iss", fs.readFileSync("./windows.iss").toString().replace(/#define MyAppVersion "(.*)"/gm, '#define MyAppVersion "' + version + '"'));
cp.execSync("wine ~/.wine/drive_c/Program\\ Files\\ \\(x86\\)/Inno\\ Setup\\ 6/ISCC.exe \"Z:\\Volumes\\Projects\\atomic\\generator\\windows-work.iss\"", { stdio: "inherit" });

console.log("# Cleaning up");
fs.renameSync("./build/ARM/Atomic.pkg", "./build/Atomic-Mac-ARM64.pkg");
fs.renameSync("./build/x64/Atomic.pkg", "./build/Atomic-Mac-x64.pkg");
fs.rmSync("./build/ARM", { recursive: true });
fs.rmSync("./build/x64", { recursive: true });
fs.rmSync("./work", { recursive: true });
fs.rmSync("./Atomic-ARM-work.pkgproj");
fs.rmSync("./Atomic-x64-work.pkgproj");
fs.rmSync("./windows-work.iss");

console.log("# Publishing to GitLab");
cp.execSync(`curl -v --header "PRIVATE-TOKEN: $(cat ~/.deploy.txt)" --header "Content-Type: multipart/form-data" --upload-file Atomic-Mac-ARM64.pkg https://source.equestria.dev/api/v4/projects/186/packages/generic/atomic/${version}/Atomic-Mac-ARM64.pkg`, { cwd: "./build" });
cp.execSync(`curl -v --header "PRIVATE-TOKEN: $(cat ~/.deploy.txt)" --header "Content-Type: multipart/form-data" --upload-file Atomic-Mac-x64.pkg https://source.equestria.dev/api/v4/projects/186/packages/generic/atomic/${version}/Atomic-Mac-x64.pkg`, { cwd: "./build" });
cp.execSync(`curl -v --header "PRIVATE-TOKEN: $(cat ~/.deploy.txt)" --header "Content-Type: multipart/form-data" --upload-file Atomic-Win32-x64.exe https://source.equestria.dev/api/v4/projects/186/packages/generic/atomic/${version}/Atomic-Win32-x64.exe`, { cwd: "./build" });
