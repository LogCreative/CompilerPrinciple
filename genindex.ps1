# run this to make the index of the doc.

Copy-Item -Path "./opg/index.html" -Destination "." -Force

(Get-Content "index.html") |
    ForEach-Object {$_ -replace "\.\./",""} |
    ForEach-Object {$_ -replace "all.html","opg/html"} |
    ForEach-Object {$_ -replace "fn.","opg/fn."} |
    ForEach-Object {$_ -replace "sidebar-","opg/sidebar-"} |
    ForEach-Object {$_ -replace "struct.","opg/struct."} |
    ForEach-Object {$_ -replace "dfs/","opg/dfs/"} |
    ForEach-Object {$_ -replace "table/","opg/table/"} |
    Out-File "index.html" -Force
