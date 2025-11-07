#   makes new day in current year
param (
    [Parameter(Mandatory)][int] $day,
    [Parameter(Mandatory)][int] $year
)

function Test-Day{
    param (
        [parameter(Mandatory)][int] $day
    )

    return ($day -ge 1 -and $day -le 25)
}

function Test-Year{
    param (
        [parameter(Mandatory)][int] $year
    )

    return ($year -ge 2015 -and $year -le (Get-Date).Year)
}

function Test-Params {
    param (
        [parameter(Mandatory)][int] $day,
        [parameter(Mandatory)][int] $year
    )

    if (!(Test-Day -day $day)) { throw "Day must be between 1 and 25." }
    if (!(Test-Year -year $year)) { throw "Year must be between 2015 and current year." }
}

function Update-MainRsDay {
    param (
        [parameter(Mandatory)][int] $day,
        [parameter(Mandatory)][int] $year
    )

    #   Modify main.rs to add new day module
    $mainRsPath = Join-Path $yearPath "src\main.rs"
    $mainRsContent = Get-Content -Path $mainRsPath
    #   if the day is already present, do nothing
    if ($mainRsContent -like "*day$twoDigitDay*") { return }
    #   find the line with the aoc_days!() macro and insert the new day inside the parentheses
    $mainRsString = $mainRsContent -join "`n"
    $regex = [regex]::new('aoc_days!\((.*?)\);',[System.Text.RegularExpressions.RegexOptions]::Singleline)
    $mainRsString = $regex.Replace($mainRsString, { param($m)
        $inner = $m.Groups[1].Value.Trim()
        if ($inner -eq '') { "aoc_days!(day$twoDigitDay);" }
        else               { "aoc_days!($inner, day$twoDigitDay);" }
    })
    $mainRsContent = $mainRsString
    Set-Content -Path $mainRsPath -Value $mainRsContent
}

function New-Year {
    param (
        [parameter(Mandatory)][int] $year
    )

    Write-Host "Year $year directory: Creating at $yearPath"

    #   Copy template folder to new year folder excluding target folder and everything inside it
    $excludeDir = Join-Path $templatePath "target"
    $robocopyArgs = @($templatePath, $yearPath, "/E", "/XD", $excludeDir)
    robocopy @robocopyArgs | Out-Null

    #   Modify Cargo.toml
    $cargoTomlPath = Join-Path $yearPath "Cargo.toml"
    $cargoTomlContent = Get-Content -Path $cargoTomlPath
    $cargoTomlContent = $cargoTomlContent -replace "XXXyearXXX", $year
    Write-Host "`tCargo.toml: $cargoTomlPath" -NoNewline
    Set-Content -Path $cargoTomlPath -Value $cargoTomlContent
    Write-Host " - Done"

    #   Create new utility file from template
    $utilTemplatePath = Join-Path $yearPath "src\utils.rs"
    $utilTemplate = Get-Content -Path $utilTemplatePath
    $utilTemplate = $utilTemplate -replace "XXXyearXXX", $year
    $utilPath = Join-Path $yearPath "src\utils.rs"
    Write-Host "`tutils.rs: $utilTemplatePath" -NoNewline
    Set-Content -Path $utilPath -Value $utilTemplate
    Write-Host " - Done"

    Write-Host "Year $year setup: Complete"
}

function New-Day {
    param (
        [parameter(Mandatory)][int] $day
    )

    Write-Host "Day $day file: Creating at $dayPath" -NoNewline
    $dayTemplatePath = Join-Path $yearPath "src\dayTemplate.rs"
    $dayTemplate = Get-Content -Path $dayTemplatePath
    $dayTemplate = $dayTemplate -replace "XXX2dayXXX", $twoDigitDay
    $dayTemplate = $dayTemplate -replace "XXXdayXXX", $day
    Write-Host " - Updating content" -NoNewline
    Set-Content -Path $dayPath -Value $dayTemplate
    Write-Host " - Updating main.rs" -NoNewline
    Update-MainRsDay -day $day -year $year
    Write-Host " - Done"
}

function New-DayInput {
    param (
        [parameter(Mandatory)][int] $day,
        [parameter(Mandatory)][int] $year
    )

    Write-Host "Day $day input: Creating at $dayInputPath" -NoNewline

    $session = New-Object Microsoft.PowerShell.Commands.WebRequestSession
    $session.UserAgent = "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/141.0.0.0 Safari/537.36 Edg/141.0.0.0"
    $session.Cookies.Add((New-Object System.Net.Cookie("_ga", "GA1.2.510995167.1760361029", "/", ".adventofcode.com")))
    $session.Cookies.Add((New-Object System.Net.Cookie("session", "<Enter Session Cookie Here>", "/", ".adventofcode.com")))
    $session.Cookies.Add((New-Object System.Net.Cookie("_gid", "GA1.2.1892417073.1761562191", "/", ".adventofcode.com")))
    $session.Cookies.Add((New-Object System.Net.Cookie("_ga_MHSNPJKWC7", "GS2.2.s1761578276`$o21`$g0`$t1761578276`$j60`$l0`$h0", "/", ".adventofcode.com")))

    $content = Invoke-WebRequest -UseBasicParsing -Uri "https://adventofcode.com/$year/day/$day/input" `
        -WebSession $session `
        -Headers @{
            "authority"="adventofcode.com"
            "method"="GET"
            "path"="/$year/day/$day/input"
            "scheme"="https"
            "accept"="text/html,application/xhtml+xml,application/xml;q=0.9,image/avif,image/webp,image/apng,*/*;q=0.8,application/signed-exchange;v=b3;q=0.7"
            "accept-encoding"="gzip, deflate, br, zstd"
            "accept-language"="en-US,en;q=0.9"
            "dnt"="1"
            "priority"="u=0, i"
            "sec-ch-ua"="`"Microsoft Edge`";v=`"141`", `"Not?A_Brand`";v=`"8`", `"Chromium`";v=`"141`""
            "sec-ch-ua-mobile"="?0"
            "sec-ch-ua-platform"="`"Windows`""
            "sec-fetch-dest"="document"
            "sec-fetch-mode"="navigate"
            "sec-fetch-site"="none"
            "sec-fetch-user"="?1"
            "upgrade-insecure-requests"="1"
        }

    Write-Host " - Writing content" -NoNewline
    New-Item -Path $dayInputPath -ItemType File -Force | Out-Null
    Set-Content -Path $dayInputPath -Value $content.Content
    Write-Host " - Done"
}

Test-Params -day $day -year $year

$templatePath = Join-Path $PSScriptRoot 'template'
$yearPath = Join-Path $PSScriptRoot "rustaoc$year"
$inputPath = Join-Path $yearPath "src\inputs"

$dayPath = if ($day -lt 10) { Join-Path $yearPath "src\day0$day.rs" }
           else             { Join-Path $yearPath "src\day$day.rs" }
$dayInputPath = if ($day -lt 10) { Join-Path $inputPath "day0$day" }
                else             { Join-Path $inputPath "day$day" }

$twoDigitDay = "{0:D2}" -f $day

if (!(Test-Path $yearPath)) { New-Year -year $year }
else                        { Write-Host "Year $year directory: Already exists at $yearPath" }
if (!(Test-Path $dayPath)) { New-Day -day $day }
else                       { Write-Host "Day $day file: Already exists at $dayPath" }
if (!(Test-Path $dayInputPath)) { New-DayInput -day $day -year $year }
else                            { Write-Host "Day $day input: Already exists at $dayInputPath" }
