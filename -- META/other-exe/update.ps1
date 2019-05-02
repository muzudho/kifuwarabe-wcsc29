# [PowerShellで日付書式にカスタム書式パターンを指定する](https://tech.guitarrapc.com/entry/2013/02/09/030226)
$oldDir = "old-$((Get-Date).ToString("yyyyMMdd-HHmm(ss)"))"

# 古いのを退避。
New-Item "./$($oldDir)" -ItemType Directory
Move-Item "./comm.log" "./$($oldDir)/comm.log"
Move-Item "./kifuwarabe-wcsc29.exe" "./$($oldDir)/kifuwarabe-wcsc29.exe"
Move-Item "./kifuwarabe-wcsc29-app-config.json" "./$($oldDir)/kifuwarabe-wcsc29-app-config.json"

# 新しいのを持ってくる。
Copy-Item "C:\muzudho\projects_rust\kifuwarabe-wcsc29\target\release\kifuwarabe-wcsc29.exe" "./kifuwarabe-wcsc29.exe"
Copy-Item "C:\muzudho\projects_rust\kifuwarabe-wcsc29\kifuwarabe-wcsc29-app-config.json" "C:\muzudho\projects_rust\kifuwarabe-wcsc29\target\release\kifuwarabe-wcsc29-app-config.json"
Copy-Item "C:\muzudho\projects_rust\kifuwarabe-wcsc29\kifuwarabe-wcsc29-app-config.json" "C:\muzudho\projects_rust\kifuwarabe-wcsc29\target\debug\kifuwarabe-wcsc29-app-config.json"
Copy-Item "C:\muzudho\projects_rust\kifuwarabe-wcsc29\kifuwarabe-wcsc29-app-config.json" "./kifuwarabe-wcsc29-app-config.json"
