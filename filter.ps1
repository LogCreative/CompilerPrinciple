Get-ChildItem . '*.tex' -Force -Recurse | ForEach-Object -Process {
    $file = [io.Path]::Combine($_.DirectoryName, $_.FullName)

    (Get-Content $file)|
        ForEach-Object { $_ -replace '***','Log Creative'} |  # Change Name
        ForEach-Object { $_ -replace '*********',''} | # Change Number
        Out-File $file -Force
}

Get-ChildItem .\ -recurse HW*.tex | ForEach-Object -Parallel {
    $file = [io.Path]::Combine($_.DirectoryName, $_.FullName)
    Set-Location $_.DirectoryName
    latexmk -pdf $file -interaction=nonstopmode -shell-escape
}

Get-ChildItem .\ -recurse *_********* | ForEach-Object {
    Rename-Item $_.FullName$_.FullName.Replace("*************","")
}