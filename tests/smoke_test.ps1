# Outbound API Rust Smoke Test
# Run this script while 'cargo run' is active.
param(
    [string]$Token,
    [string]$BaseUrl = "http://localhost:8081/api/v1",
    [string]$Endpoint,
    [string]$Payload
)

$fallbackToken = "eyJhbGciOiJub25lIiwidHlwIjoiSldUIn0.eyJzdWIiOiIxMjM0NTY3ODkwIiwibmFtZSI6IkpvaG4gRG9lIiwiaWF0IjoxNTE2MjM5MDIyLCJyb2xlcyI6WyJEYXRhUmVhZGVyIl0sImF1ZCI6IjMzNjU2NTg1LTdhYTMtNGZjMS04ODFhLWU5Yzk2OTNkYWUwMCIsImlzcyI6Imh0dHBzOi8vbG9naW4ubWljcm9zb2Z0b25saW5lLmNvbS84NjE5YzY3Yy05NDVhLTQ4YWUtOGU3Ny0zNWIxYjcxYzlIOTgvdjIuMCJ9."
$activeToken = if ([string]::IsNullOrWhiteSpace($Token)) { $fallbackToken } else { $Token }

function Test-Endpoint {
    param($Name, $Method, $Path, $Body)
    
    $fullUri = if ($Path.StartsWith("http")) { $Path } else { "$BaseUrl$Path" }
    Write-Host "`n[TEST] $Name ($Method $fullUri)..." -ForegroundColor Cyan
    
    $params = @{
        Uri = $fullUri
        Method = $Method
        Headers = @{
            "Authorization" = "Bearer $activeToken"
            "Content-Type" = "application/json"
        }
        ErrorAction = "SilentlyContinue"
    }
    
    if ($Body) { $params.Body = $Body }

    $startTime = Get-Date
    try {
        $response = Invoke-RestMethod @params -ResponseHeadersVariable headers
        $elapsed = ((Get-Date) - $startTime).TotalMilliseconds
        Write-Host "  STATUS: 200 OK ($($elapsed)ms)" -ForegroundColor Green
        if ($response) { 
            $count = if ($response.Count) { $response.Count } else { 1 }
            Write-Host "  RESULT: Received $count items" -ForegroundColor Gray
        }
        return $true
    } catch {
        $status = $_.Exception.Response.StatusCode.value__
        Write-Host "  FAILED: Status $status" -ForegroundColor Red
        if ($_.ErrorDetails.Message) { Write-Host "  ERROR: $($_.ErrorDetails.Message)" -ForegroundColor DarkGray }
        return $false
    }
}

if (![string]::IsNullOrWhiteSpace($Endpoint)) {
    Write-Host "Executing Custom Request..." -Style Bold
    
    $activePayload = $Payload
    if (![string]::IsNullOrWhiteSpace($Payload) -and (Test-Path $Payload -PathType Leaf)) {
        Write-Host "  INFO: Loading payload from file: $Payload" -ForegroundColor Gray
        $activePayload = Get-Content -Path $Payload -Raw
    }

    Test-Endpoint "Custom Request" "POST" $Endpoint $activePayload
    exit
}

Write-Host "Starting Default Operational Smoke Test Suite..." -Style Bold

# 1. Health Checks
Test-Endpoint "Liveness Check" "GET" "/health/liveness"
Test-Endpoint "Readiness Check" "GET" "/health/readiness"

# 2. Transactional Data (Standard)
$txBody = @(
    @{
        Ids = @(536000751)
        Filters = @{
            Expressions = @("ReferenceTime = 2024-01-01T00:00:00")
        }
    }
) | ConvertTo-Json -Depth 5

Test-Endpoint "Transactional Sync Request" "POST" "/productive/timeseries" $txBody

# 3. Transactional Data (Streaming)
Test-Endpoint "Transactional Stream Request" "POST" "/productive/timeseries/streaming" $txBody

Write-Host "`nSmoke Test Suite Complete." -ForegroundColor Yellow
