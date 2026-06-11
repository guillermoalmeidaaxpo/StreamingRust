# Outbound API Rust Smoke Test
# Run this script while 'cargo run' is active.

$baseUrl = "http://localhost:8080/api/v1"
$dummyToken = "eyJhbGciOiJub25lIiwidHlwIjoiSldUIn0.eyJzdWIiOiIxMjM0NTY3ODkwIiwibmFtZSI6IkpvaG4gRG9lIiwiaWF0IjoxNTE2MjM5MDIyLCJyb2xlcyI6WyJEYXRhUmVhZGVyIl0sImF1ZCI6IjMzNjU2NTg1LTdhYTMtNGZjMS04ODFhLWU5Yzk2OTNkYWUwMCIsImlzcyI6Imh0dHBzOi8vbG9naW4ubWljcm9zb2Z0b25saW5lLmNvbS84NjE5YzY3Yy05NDVhLTQ4YWUtOGU3Ny0zNWIxYjcxYzlIOTgvdjIuMCJ9."

function Test-Endpoint {
    param($Name, $Method, $Path, $Body)
    
    Write-Host "`n[TEST] $Name ($Method $Path)..." -ForegroundColor Cyan
    
    $params = @{
        Uri = "$baseUrl$Path"
        Method = $Method
        Headers = @{
            "Authorization" = "Bearer $dummyToken"
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
        return $true
    } catch {
        $status = $_.Exception.Response.StatusCode.value__
        Write-Host "  FAILED: Status $status" -ForegroundColor Red
        if ($_.ErrorDetails.Message) { Write-Host "  ERROR: $($_.ErrorDetails.Message)" -ForegroundColor DarkGray }
        return $false
    }
}

Write-Host "Starting Operational Smoke Test for Rust Outbound API..." -Style Bold

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

# 4. CSV Generic
Test-Endpoint "Generic CSV Request" "POST" "/generic" $txBody

Write-Host "`nSmoke Test Complete." -ForegroundColor Yellow
