# Arden Dev Shell (auto-unblock edition)
Set-Location "D:\ae\arden"
Write-Host "`n=== Arden Dev Shell ===" -ForegroundColor Cyan
Write-Host "Location: D:\ae\arden" -ForegroundColor DarkCyan

# 🧹 Снятие меток «из интернета»
$unblocked = 0
Get-ChildItem -Recurse -Force -Include *.ps1,*.rs,*.toml,*.md,*.ron |
    ForEach-Object {
        try {
            if (Get-Item $_.FullName -Stream "Zone.Identifier" -ErrorAction SilentlyContinue) {
                Unblock-File $_.FullName
                $unblocked++
            }
        } catch {}
    }
if ($unblocked -gt 0) {
    Write-Host "Removed Internet zone marks from $unblocked file(s)." -ForegroundColor Yellow
} else {
    Write-Host "No Internet zone marks detected." -ForegroundColor Green
}

# 🔗 Алиасы и PATH
Set-Alias run 'cargo run'
Set-Alias build 'cargo build'
Set-Alias check 'cargo check'
Set-Alias g 'git'
Set-Alias gs 'git status'
Set-Alias gcmt 'git commit -m'
Set-Alias gph 'git push'
$env:PATH += ";D:\ae\CodexCLI;D:\ae\tools"

Write-Host "Aliases loaded. Ready for Rust & Codex." -ForegroundColor Green
