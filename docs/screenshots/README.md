# Screenshots needed here

The main README references two images that aren't committed yet (couldn't be captured automatically — see below). Add these two files to make the README render correctly:

- `main.png` — the main window: sidebar with a couple of groups/connections expanded, one connection selected showing the General tab
- `advanced.png` — a connection open on the **Advanced** tab (or **Display**, if that looks better)

## How to capture them

1. Run the app: `pnpm tauri dev` (or open the built `.app`/`.exe`)
2. Create a couple of demo groups/connections so the sidebar isn't empty — use fake hosts, not real servers, since this repo is public
3. macOS: `Cmd+Shift+4`, then `Space`, then click the RDP Manager window — this saves a clean screenshot of just the window (no need to crop)
   Windows: `Alt+PrtScn` captures just the focused window
4. Save as `main.png` / `advanced.png` in this folder and commit
