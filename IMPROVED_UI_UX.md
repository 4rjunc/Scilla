
# SCILLA TUI - Improved UI/UX Design

## Overview of Improvements

### Key Enhancements:
- **Visual Hierarchy**: Color-coded sections, icons, and better spacing
- **Contextual Help**: Inline help text and command previews
- **Breadcrumb Navigation**: Always know where you are
- **Quick Actions**: Frequently used commands accessible via shortcuts
- **Smart Defaults**: Pre-filled values based on context
- **Progress Indicators**: Better feedback for long operations
- **Search & Filter**: Quick command finder
- **History**: Recently used commands for faster access
- **Validation**: Real-time input validation with helpful errors

---

## Main Dashboard (Enhanced)

```
┌──────────────────────────────────────────────────────────────────────────────┐
│  ⚡ SCILLA v1.2.0              8vDg..Er6k | devnet │ Connected ● │ 12:34:56  │
├──────────────────────────────────────────────────────────────────────────────┤
│  Epoch: 234 │ Slot: 123,456,789 │ Balance: 12.5 SOL │ TPS: 2,341 │ Health: ●│
├──────────────────────────────────────────────────────────────────────────────┤
│                                                                              │
│  ┌─ Quick Actions ─────────────────────┐  ┌─ Recent Commands ──────────┐   │
│  │                                      │  │                             │   │
│  │  [1] Create Vote Account             │  │  • Create Vote Account      │   │
│  │  [2] Delegate Stake                  │  │  • View Balance             │   │
│  │  [3] View Account Balance            │  │  • Switch Cluster           │   │
│  │  [4] Check Validator Status          │  │                             │   │
│  │                                      │  └─────────────────────────────┘   │
│  └──────────────────────────────────────┘                                    │
│                                                                              │
│  ┌─ Command Groups ──────────────────────────────────────────────────────┐   │
│  │                                                                        │   │
│  │   ▸ 💰 Account Management              Manage wallets and balances    │   │
│  │   ▸ 🌐 Cluster Operations              Switch networks and endpoints  │   │
│  │   ▸ 🔒 Stake Management                Delegate and manage stakes     │   │
│  │   ▸ 🗳️  Vote Account                    Validator voting operations   │   │
│  │   ▸ ⚙️  Configuration                   System settings and keys       │   │
│  │   ▸ 📊 Monitoring                      View stats and health          │   │
│  │                                                                        │   │
│  └────────────────────────────────────────────────────────────────────────┘   │
│                                                                              │
├──────────────────────────────────────────────────────────────────────────────┤
│  [1-9] Quick Action  [↑↓] Navigate  [Enter] Select  [/] Search  [h] Help [q] Quit │
└──────────────────────────────────────────────────────────────────────────────┘
```

**Improvements:**
- Icons for visual scanning
- Quick actions with number shortcuts
- Recent commands for efficiency
- Better status indicators with real TPS and health
- Descriptive text for each command group

---

## Quick Search/Filter (NEW)

```
┌──────────────────────────────────────────────────────────────────────────────┐
│  ⚡ SCILLA v1.2.0              8vDg..Er6k | devnet │ Connected ● │ 12:34:56  │
├──────────────────────────────────────────────────────────────────────────────┤
│  Home                                                                        │
├──────────────────────────────────────────────────────────────────────────────┤
│                                                                              │
│  ┌─ Search Commands ────────────────────────────────────────────────────┐    │
│  │                                                                       │    │
│  │  🔍 vote█                                                             │    │
│  │                                                                       │    │
│  │  ┌─ Matching Commands (3) ──────────────────────────────────────┐    │    │
│  │  │                                                               │    │    │
│  │  │  > 🗳️  Create Vote Account         [Vote Account]             │    │    │
│  │  │    🗳️  Show Vote Account Info      [Vote Account]             │    │    │
│  │  │    🗳️  Update Vote Account         [Vote Account]             │    │    │
│  │  │                                                               │    │    │
│  │  └───────────────────────────────────────────────────────────────┘    │    │
│  │                                                                       │    │
│  └───────────────────────────────────────────────────────────────────────┘    │
│                                                                              │
│  💡 Tip: Use fuzzy search - try "cva" for "Create Vote Account"              │
│                                                                              │
├──────────────────────────────────────────────────────────────────────────────┤
│  [Type] Search  [↑↓] Navigate  [Enter] Select  [Esc] Cancel                  │
└──────────────────────────────────────────────────────────────────────────────┘
```

**New Feature:**
- Fuzzy search for quick command access
- Shows which group commands belong to
- Contextual tips for power users

---

## Submenu with Breadcrumbs

```
┌──────────────────────────────────────────────────────────────────────────────┐
│  ⚡ SCILLA v1.2.0              8vDg..Er6k | devnet │ Connected ● │ 12:34:56  │
├──────────────────────────────────────────────────────────────────────────────┤
│  Home > Vote Account                                                         │
├──────────────────────────────────────────────────────────────────────────────┤
│                                                                              │
│  ┌─ Vote Account Operations ─────────────────────────────────────────────┐   │
│  │                                                                        │   │
│  │   > Create Vote Account                Create a new vote account      │   │
│  │     Show Vote Account Info             Display account details        │   │
│  │     Update Vote Commission             Modify commission rate         │   │
│  │     Withdraw from Vote Account         Extract funds                  │   │
│  │     Close Vote Account                 Permanently close account      │   │
│  │                                                                        │   │
│  │   ← Back to Main Menu                                                 │   │
│  │                                                                        │   │
│  └────────────────────────────────────────────────────────────────────────┘   │
│                                                                              │
│  ℹ️  Vote accounts are required for validators to participate in consensus   │
│                                                                              │
├──────────────────────────────────────────────────────────────────────────────┤
│  [↑↓] Navigate  [Enter] Select  [Backspace] Back  [h] Help  [q] Quit         │
└──────────────────────────────────────────────────────────────────────────────┘
```

**Improvements:**
- Breadcrumb navigation shows current location
- Descriptions for each action
- Contextual information panel
- Backspace for quick back navigation

---

## Enhanced Form with Validation

```
┌──────────────────────────────────────────────────────────────────────────────┐
│  ⚡ SCILLA v1.2.0              8vDg..Er6k | devnet │ Connected ● │ 12:34:56  │
├──────────────────────────────────────────────────────────────────────────────┤
│  Home > Vote Account > Create Vote Account                                   │
├──────────────────────────────────────────────────────────────────────────────┤
│                                                                              │
│    ┌─ Create Vote Account ──────────────────────────────────────────────┐    │
│    │                                                               [3/4] │    │
│    │  Vote Keypair:       ✓ ./vote-keypair.json                         │    │
│    │                                                                     │    │
│    │  Identity Keypair:   ✓ ./validator-keypair.json                    │    │
│    │                                                                     │    │
│    │  Withdraw Authority: [./withdraw-key█                ]             │    │
│    │                       ┌─────────────────────────────┐              │    │
│    │                       │ ./withdraw-keypair.json     │              │    │
│    │                       │ ./withdraw-backup.json      │              │    │
│    │                       │ ./cold-wallet.json          │              │    │
│    │                       └─────────────────────────────┘              │    │
│    │                                                                     │    │
│    │  Commission (%):     [0                             ]              │    │
│    │                       ℹ️  0-100, typical: 5-10%                     │    │
│    │                                                                     │    │
│    │  💡 Tip: Use a separate cold wallet for withdraw authority         │    │
│    │                                                                     │    │
│    │  ┌─ Estimated Cost ─────────────────────────────────────────────┐  │    │
│    │  │  Rent: ~0.0282 SOL │ Transaction: ~0.00001 SOL              │  │    │
│    │  └────────────────────────────────────────────────────────────────┘  │    │
│    │                                                                     │    │
│    │               [Submit]            [Cancel]                          │    │
│    │                                                                     │    │
│    └─────────────────────────────────────────────────────────────────────┘    │
│                                                                              │
├──────────────────────────────────────────────────────────────────────────────┤
│  [Tab] Next/Autocomplete  [Shift+Tab] Previous  [Enter] Submit  [Esc] Cancel │
└──────────────────────────────────────────────────────────────────────────────┘
```

**Improvements:**
- Progress indicator (3/4 fields completed)
- Checkmarks for completed fields
- Inline help text with typical values
- Cost estimation before submitting
- Better visual separation
- Contextual tips

---

## Form Validation Error

```
┌──────────────────────────────────────────────────────────────────────────────┐
│  ⚡ SCILLA v1.2.0              8vDg..Er6k | devnet │ Connected ● │ 12:34:56  │
├──────────────────────────────────────────────────────────────────────────────┤
│  Home > Vote Account > Create Vote Account                                   │
├──────────────────────────────────────────────────────────────────────────────┤
│                                                                              │
│    ┌─ Create Vote Account ──────────────────────────────────────────────┐    │
│    │                                                               [2/4] │    │
│    │  Vote Keypair:       ✓ ./vote-keypair.json                         │    │
│    │                                                                     │    │
│    │  Identity Keypair:   [./invalid-key.txt                 ]          │    │
│    │                       ❌ Invalid keypair file format                │    │
│    │                       💡 Must be a valid JSON keypair file          │    │
│    │                                                                     │    │
│    │  Withdraw Authority: [                              ]              │    │
│    │                                                                     │    │
│    │  Commission (%):     [150                           ]              │    │
│    │                       ❌ Must be between 0-100                      │    │
│    │                                                                     │    │
│    └─────────────────────────────────────────────────────────────────────┘    │
│                                                                              │
├──────────────────────────────────────────────────────────────────────────────┤
│  [Tab] Next Field  [Enter] Accept  [Esc] Cancel                              │
└──────────────────────────────────────────────────────────────────────────────┘
```

**New Feature:**
- Real-time validation with clear error messages
- Helpful suggestions to fix errors
- Visual indicators (✓ ❌) for field status

---

## Loading State with Progress

```
┌──────────────────────────────────────────────────────────────────────────────┐
│  ⚡ SCILLA v1.2.0              8vDg..Er6k | devnet │ Connected ● │ 12:34:56  │
├──────────────────────────────────────────────────────────────────────────────┤
│  Home > Vote Account > Create Vote Account                                   │
├──────────────────────────────────────────────────────────────────────────────┤
│                                                                              │
│                                                                              │
│                     ┌───────────────────────────────────┐                    │
│                     │  ⏳ Creating Vote Account...      │                    │
│                     ├───────────────────────────────────┤                    │
│                     │                                   │                    │
│                     │  [████████████░░░░░░] 65%         │                    │
│                     │                                   │                    │
│                     │  ✓ Validating keypairs            │                    │
│                     │  ✓ Checking account balance       │                    │
│                     │  ⟳ Sending transaction...         │                    │
│                     │  ○ Confirming on-chain            │                    │
│                     │                                   │                    │
│                     │  Estimated time: 8s remaining     │                    │
│                     │                                   │                    │
│                     └───────────────────────────────────┘                    │
│                                                                              │
│                                                                              │
├──────────────────────────────────────────────────────────────────────────────┤
│  ⏳ Processing... Please wait                                                │
└──────────────────────────────────────────────────────────────────────────────┘
```

**Improvements:**
- Multi-step progress indicator
- Shows which steps are complete/in-progress
- Time estimate for better UX
- Can't be cancelled accidentally

---

## Success Screen with Actions

```
┌──────────────────────────────────────────────────────────────────────────────┐
│  ⚡ SCILLA v1.2.0              8vDg..Er6k | devnet │ Connected ● │ 12:34:56  │
├──────────────────────────────────────────────────────────────────────────────┤
│  Home > Vote Account > Create Vote Account                                   │
├──────────────────────────────────────────────────────────────────────────────┤
│                                                                              │
│                     ┌───────────────────────────────────────────┐            │
│                     │  ✅ Vote Account Created Successfully!    │            │
│                     ├───────────────────────────────────────────┤            │
│                     │                                           │            │
│                     │  Vote Account:                            │            │
│                     │  GxN9...kR2p                              │            │
│                     │  [Copy]                                   │            │
│                     │                                           │            │
│                     │  Transaction Signature:                   │            │
│                     │  5xK3j8fL...h2s9mNp                       │            │
│                     │  [Copy] [View on Explorer]                │            │
│                     │                                           │            │
│                     │  ┌─ Details ────────────────────────┐    │            │
│                     │  │  Commission: 5%                  │    │            │
│                     │  │  Rent Paid: 0.0282 SOL           │    │            │
│                     │  │  Status: Active                  │    │            │
│                     │  └──────────────────────────────────┘    │            │
│                     │                                           │            │
│                     │  ┌─ Next Steps ──────────────────────┐   │            │
│                     │  │  [1] Delegate stake to account    │   │            │
│                     │  │  [2] View account details         │   │            │
│                     │  │  [3] Return to main menu          │   │            │
│                     │  └───────────────────────────────────┘   │            │
│                     │                                           │            │
│                     └───────────────────────────────────────────┘            │
│                                                                              │
├──────────────────────────────────────────────────────────────────────────────┤
│  [1-3] Quick Action  [Enter] Continue  [c] Copy  [e] Explorer                │
└──────────────────────────────────────────────────────────────────────────────┘
```

**Improvements:**
- Actionable next steps
- Copy buttons for addresses/signatures
- Link to explorer
- Transaction details visible
- Suggested workflows for efficiency

---

## Error Screen with Recovery

```
┌──────────────────────────────────────────────────────────────────────────────┐
│  ⚡ SCILLA v1.2.0              8vDg..Er6k | devnet │ Connected ● │ 12:34:56  │
├──────────────────────────────────────────────────────────────────────────────┤
│  Home > Vote Account > Create Vote Account                                   │
├──────────────────────────────────────────────────────────────────────────────┤
│                                                                              │
│                     ┌───────────────────────────────────────────┐            │
│                     │  ❌ Transaction Failed                     │            │
│                     ├───────────────────────────────────────────┤            │
│                     │                                           │            │
│                     │  Error: Insufficient funds                │            │
│                     │                                           │            │
│                     │  Required: 0.0282 SOL (rent)              │            │
│                     │  Available: 0.0100 SOL                    │            │
│                     │  Needed: 0.0182 SOL more                  │            │
│                     │                                           │            │
│                     │  ┌─ Suggested Actions ───────────────┐   │            │
│                     │  │                                    │   │            │
│                     │  │  [1] Request airdrop (devnet)     │   │            │
│                     │  │  [2] Check account balance        │   │            │
│                     │  │  [3] Retry transaction            │   │            │
│                     │  │  [4] Cancel and return            │   │            │
│                     │  │                                    │   │            │
│                     │  └────────────────────────────────────┘   │            │
│                     │                                           │            │
│                     │  📋 Error copied to clipboard             │            │
│                     │                                           │            │
│                     └───────────────────────────────────────────┘            │
│                                                                              │
├──────────────────────────────────────────────────────────────────────────────┤
│  [1-4] Quick Action  [r] Retry  [Esc] Back                                   │
└──────────────────────────────────────────────────────────────────────────────┘
```

**Improvements:**
- Clear error explanation
- Specific numbers showing the problem
- Actionable recovery suggestions
- Context-aware help (airdrop on devnet)
- Auto-copy error for support

---

## Help Overlay (NEW)

```
┌──────────────────────────────────────────────────────────────────────────────┐
│  ⚡ SCILLA - Keyboard Shortcuts                                    [Press h or Esc to close] │
├──────────────────────────────────────────────────────────────────────────────┤
│                                                                              │
│  ┌─ Navigation ──────────────┐  ┌─ Actions ───────────────────┐             │
│  │                           │  │                              │             │
│  │  ↑/k       Move up        │  │  Enter     Select/Confirm    │             │
│  │  ↓/j       Move down      │  │  Space     Toggle/Select     │             │
│  │  ←/h       Go back        │  │  Esc       Cancel/Back       │             │
│  │  →/l       Go forward     │  │  q         Quit application  │             │
│  │  Home      Jump to top    │  │  c         Copy to clipboard │             │
│  │  End       Jump to bottom │  │  e         Open in explorer  │             │
│  │                           │  │                              │             │
│  └───────────────────────────┘  └──────────────────────────────┘             │
│                                                                              │
│  ┌─ Quick Access ───────────────────────────────────────────────────────┐    │
│  │                                                                       │    │
│  │  /          Search commands         Ctrl+R    Refresh status         │    │
│  │  1-9        Quick actions           Ctrl+L    View logs              │    │
│  │  Tab        Next field/Autocomplete Ctrl+H    Command history        │    │
│  │  Shift+Tab  Previous field          ?         Show this help         │    │
│  │                                                                       │    │
│  └───────────────────────────────────────────────────────────────────────┘    │
│                                                                              │
│  💡 Tip: Most commands support both arrow keys and vim-style (hjkl) navigation│
│                                                                              │
├──────────────────────────────────────────────────────────────────────────────┤
│  [h] or [Esc] Close Help                                                     │
└──────────────────────────────────────────────────────────────────────────────┘
```

**New Feature:**
- Comprehensive keyboard shortcuts
- Always accessible via 'h' key
- Vim-style navigation support

---

## Command History (NEW)

```
┌──────────────────────────────────────────────────────────────────────────────┐
│  ⚡ SCILLA v1.2.0              8vDg..Er6k | devnet │ Connected ● │ 12:34:56  │
├──────────────────────────────────────────────────────────────────────────────┤
│  Home > History                                                              │
├──────────────────────────────────────────────────────────────────────────────┤
│                                                                              │
│  ┌─ Command History (Last 10) ───────────────────────────────────────────┐   │
│  │                                                                        │   │
│  │  12:32  ✓  Create Vote Account          GxN9...kR2p                   │   │
│  │  12:15  ✓  Check Balance                12.5 SOL                      │   │
│  │  12:10  ✓  Switch Cluster               devnet                        │   │
│  │  11:45  ❌ Create Stake Account          Error: Insufficient funds     │   │
│  │  11:30  ✓  Request Airdrop              2.0 SOL                       │   │
│  │                                                                        │   │
│  │  > Select a command to view details or re-run                         │   │
│  │                                                                        │   │
│  └────────────────────────────────────────────────────────────────────────┘   │
│                                                                              │
├──────────────────────────────────────────────────────────────────────────────┤
│  [↑↓] Navigate  [Enter] View Details  [r] Re-run  [d] Delete  [Esc] Back    │
└──────────────────────────────────────────────────────────────────────────────┘
```

**New Feature:**
- View command history
- Re-run previous commands
- See success/failure status

---

## Updated Architecture

```
┌─────────────────────────────────────────────────────────────────┐
│                         Enhanced App State                      │
├─────────────────────────────────────────────────────────────────┤
│  current_screen: Screen (enum with breadcrumb trail)            │
│  selected_index: usize                                          │
│  form_data: HashMap<String, FormField> (with validation state)  │
│  cluster_info: ClusterInfo (auto-refresh every 5s)              │
│  wallet_balance: u64 (auto-refresh every 5s)                    │
│  search_query: String (for fuzzy search)                        │
│  command_history: Vec<CommandRecord> (last 50 commands)         │
│  notifications: Vec<Notification> (toast messages)              │
│  help_visible: bool (overlay state)                             │
└─────────────────────────────────────────────────────────────────┘
```

---

## Enhanced Event Loop

```
┌─────────────────────────────────────────────────────────────────┐
│                    Event-Driven Architecture                    │
├─────────────────────────────────────────────────────────────────┤
│  1. Poll keyboard events (with debounce)                        │
│  2. Poll background refresh tasks                               │
│  3. Process validation events (async)                           │
│  4. Update state with proper error handling                     │
│  5. Render UI with dirty-checking (only changed regions)        │
│  6. Handle notifications/toasts                                 │
│  7. Repeat (60 FPS target)                                      │
└─────────────────────────────────────────────────────────────────┘
```

---

## Expanded State Machine

```
        ┌──────────┐
        │Dashboard │ ◄──────────────────┐
        │(enhanced)│                    │
        └────┬─────┘                    │
             │                          │
     ┌───────┼───────┬───────────┐      │
     │       │       │           │      │
     ▼       ▼       ▼           ▼      │
┌────────┐┌──────┐┌────────┐┌─────────┐│
│ Search ││ Help ││History ││Submenu  ││
│(fuzzy) ││(ovly)││(list)  ││(groups) ││
└────────┘└──────┘└────────┘└────┬────┘│
                                 │     │
             Esc ────────────────┘     │
             Enter                     │
                  ▼                    │
             ┌─────────┐               │
             │  Form   │ ──── Esc ─────┤
             │(validtn)│               │
             └────┬────┘               │
                  │ Submit             │
                  ▼                    │
             ┌─────────┐               │
             │ Loading │               │
             │(progress)               │
             └────┬────┘               │
                  │                    │
         ┌────────┴────────┐           │
         ▼                 ▼           │
    ┌─────────┐       ┌─────────┐     │
    │ Success │       │  Error  │     │
    │(actions)│       │(recovery)      │
    └────┬────┘       └────┬────┘     │
         │                 │           │
         └─────────┬───────┘           │
                   │ Enter             │
                   └───────────────────┘
```

---

## Summary of UX Improvements

### 1. **Discoverability**
- Icons and descriptions make features obvious
- Search helps users find commands quickly
- Contextual tips guide new users

### 2. **Efficiency**
- Quick actions with number shortcuts
- Command history for repeated tasks
- Smart defaults reduce typing

### 3. **Feedback**
- Progress indicators for long operations
- Real-time validation prevents errors
- Clear success/error states

### 4. **Navigation**
- Breadcrumbs show current location
- Multiple back options (Esc, Backspace, ←)
- Help always accessible

### 5. **Error Prevention & Recovery**
- Validation before submission
- Cost estimation prevents surprises
- Clear error messages with solutions

### 6. **Visual Design**
- Icons for quick scanning
- Color-coded status (●)
- Better spacing and hierarchy

### 7. **Accessibility**
- Vim-style navigation support
- Screen reader friendly (with proper labels)
- Keyboard-first design

---

## Implementation Priority

### Phase 1 (MVP)
- Enhanced dashboard with icons
- Breadcrumb navigation
- Real-time validation
- Improved error messages

### Phase 2 (Enhanced)
- Search/filter functionality
- Command history
- Progress indicators
- Cost estimation

### Phase 3 (Polish)
- Help overlay
- Quick actions
- Recent commands
- Fuzzy search

---

## Technical Considerations

### Libraries to Consider:
- **ratatui**: Modern TUI framework (vs tui-rs)
- **crossterm**: Better cross-platform support
- **fuzzy-matcher**: For search functionality
- **serde**: State persistence between sessions

### Performance:
- Dirty-checking for partial redraws
- Debounce keyboard input
- Async validation to avoid blocking
- Background refresh without UI freezing

### Testing:
- Unit tests for validation logic
- Integration tests for state machine
- Snapshot tests for UI rendering
