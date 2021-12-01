function New-Day {
    param  (
        [Parameter(Mandatory)]
        [int] $Year,

        [Parameter(Mandatory)]
        [int] $Day
    )

    if ( !(Test-Path -Path "./$Year") )
    {
        New-Item -Path "./$Year" -ItemType Directory
    }
    if ( !(Test-Path -Path "./$Year/day$Day") )
    {
        cargo new --lib "./$Year/day$Day"
        New-Item -Path "./$Year/day$Day/assets" -ItemType Directory
        New-Item -Path "./$Year/day$Day/assets/input.txt" -ItemType File
        New-Item -Path "./$Year/day$Day/assets/test.txt" -ItemType File
        New-Item -Path "./$Year/day$Day/assets/input_ludegra.txt" -ItemType File
        New-Item -Path "./$Year/day$Day/src/part1.rs" -ItemType File
        New-Item -Path "./$Year/day$Day/src/part2.rs" -ItemType File
        New-Item -Path "./$Year/day$Day/src/structs.rs" -ItemType File
        Add-Content -Path "./$Year/day$Day/cargo.toml" -Value "utils = { path = `"../../utils`" }"
    }
}