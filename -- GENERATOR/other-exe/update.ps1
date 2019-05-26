# [PowerShell�œ��t�����ɃJ�X�^�������p�^�[�����w�肷��](https://tech.guitarrapc.com/entry/2013/02/09/030226)
$oldDir = "old-$((Get-Date).ToString("yyyyMMdd-HHmm(ss)"))"

# �Â��̂�ޔ��B
New-Item "./$($oldDir)" -ItemType Directory
Move-Item "./comm.log" "./$($oldDir)/comm.log"
Move-Item "./kifuwarabe-wcsc29.exe" "./$($oldDir)/kifuwarabe-wcsc29.exe"
Move-Item "./kifuwarabe-wcsc29-exe-config.json" "./$($oldDir)/kifuwarabe-wcsc29-exe-config.json"

# �V�����̂������Ă���B
Copy-Item "C:\muzudho\projects_rust\kifuwarabe-wcsc29\target\release\kifuwarabe-wcsc29.exe" "./kifuwarabe-wcsc29.exe"
Copy-Item "C:\muzudho\projects_rust\kifuwarabe-wcsc29\kifuwarabe-wcsc29-exe-config.json" "C:\muzudho\projects_rust\kifuwarabe-wcsc29\target\release\kifuwarabe-wcsc29-exe-config.json"
Copy-Item "C:\muzudho\projects_rust\kifuwarabe-wcsc29\kifuwarabe-wcsc29-exe-config.json" "C:\muzudho\projects_rust\kifuwarabe-wcsc29\target\debug\kifuwarabe-wcsc29-exe-config.json"
Copy-Item "C:\muzudho\projects_rust\kifuwarabe-wcsc29\kifuwarabe-wcsc29-exe-config.json" "./kifuwarabe-wcsc29-exe-config.json"
