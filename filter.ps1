Get-ChildItem . '*.tex' -Force -Recurse | ForEach-Object -Process {
    $file = [io.Path]::Combine($_.DirectoryName, $_.FullName)

    (Get-Content $file)|
        ForEach-Object { $_ -replace '李子龙','Log Creative'} |  # Change Name
        ForEach-Object { $_ -replace '518070910095',''} | # Change Number
        Out-File $file -Force
}

Get-ChildItem .\ -recurse HW*.tex | ForEach-Object -Parallel {
    $file = [io.Path]::Combine($_.DirectoryName, $_.FullName)
    Set-Location $_.DirectoryName
    latexmk -pdf $file -interaction=nonstopmode -shell-escape
}

Get-ChildItem .\ -recurse *_518070910095* | ForEach-Object {
    Rename-Item $_.FullName$_.FullName.Replace("\_李子龙\_518070910095","")
}