
# SCILLA TUI - Simplified Design (lazygit-inspired)

## Design Philosophy

- **Number keys (1-5)** to select options instantly
- **Single letter commands** for common actions
- **Minimal visual noise** - clean, scannable interface
- **Zero cognitive load** - obvious what each key does
- **Flat navigation** - less nesting, faster access

---

## Main Menu

### Scilla

```
⚡ Scilla — Hacking Through the Solana Matrix
Using Scilla config path : "/Users/arjunc/.config/scilla.toml"
? Choose a command group:
> Account
  Cluster
  Stake
  Vote
  ScillaConfig
  Exit
[↑↓ to move, enter to select, type to filter]
```


```
┌─ LAZY-SCILLA ──────────────────────────────────────────────────────────────┐
│ devnet │ 12.5 SOL │ 8EwoWotLUEipf2rAtje738n6NX3LkhGKbtBCx9Z4RBDb           │
├────────────────────────────────────────────────────────────────────────────┤
│                                                            [Main Menu]     │
│  1  Account                                                                │
│  2  Cluster                                                                │
│  3  Stake                                                                  │
│  4  Vote                                                                   │
│  5  Scilla Config                                                          │
│                                                                            │
├────────────────────────────────────────────────────────────────────────────┤
│ [1-n] select  [\] search  [n] network [esc] back  [?] help  [q] quit       │
└────────────────────────────────────────────────────────────────────────────┘
```

**Key Features:**
- Just press `1` to select `Account`. Which move to account realted options - no Enter needed
- `n` changes network from devnet -> mainnet -> testnet -> devnet .. repeat
- `?` displays help box
- `\` display a search bar to search commands
- All primary actions visible on one screen
- Clean, minimal status bar


## Account

```
> Choose a command group: Account
? Account Command:
> Fetch account
  Check balance
  Transfer SOL
  Request airdrop
  Check transaction confirmation
  View largest accounts
  View nonce account
[↑↓ to move, enter to select, type to filter]
```

```
┌─ LAZY-SCILLA ──────────────────────────────────────────────────────────────┐
│ devnet │ 12.5 SOL │ 8EwoWotLUEipf2rAtje738n6NX3LkhGKbtBCx9Z4RBDb           │
├────────────────────────────────────────────────────────────────────────────┤
│                                                            [Account Menu]  │
│  1  Fetch Account                                                          │
│  2  Check Balance                                                          │
│  3  Transfer SOL                                                           │
│  4  Request Airdrop                                                        │
│  5  Check transaction confirmation                                         │
│  6  View largest accounts                                                  │
│  7  View nonce account                                                     │
│                                                                            │
├────────────────────────────────────────────────────────────────────────────┤
│ [1-n] select  [\] search  [n] network [esc] back  [?] help  [q] quit       │
└────────────────────────────────────────────────────────────────────────────┘
```

- `esc` to back to previous screen

### Fetch Account

```
> Choose a command group: Account
> Account Command: Fetch account
? Enter Pubkey:

```


```
┌─ LAZY-SCILLA ──────────────────────────────────────────────────────────────┐
│ devnet │ 12.5 SOL │ 8EwoWotLUEipf2rAtje738n6NX3LkhGKbtBCx9Z4RBDb           │
├────────────────────────────────────────────────────────────────────────────┤
│                                            [Account Menu > Fetch Account]  │
│                                                                            │
│                                                                            │
│                               Account Address:                             │
│                                                                            │
│           ┌────────────────────────────────────────────────────────────┐   │
│           │_                                                           │   │
│           └────────────────────────────────────────────────────────────┘   │
│                                                                            │
├────────────────────────────────────────────────────────────────────────────┤
│ [1-n] select  [\] search  [n] network [esc] back  [?] help  [q] quit       │
└────────────────────────────────────────────────────────────────────────────┘
```

---

## Create Vote Account

```
┌─ SCILLA > Create Vote Account ─────────────────────────────────────────────┐
│ devnet │ 12.5 SOL │ 8EwoWotLUEipf2rAtje738n6NX3LkhGKbtBCx9Z4RBDb           │
├────────────────────────────────────────────────────────────────────────────┤
│                                                                            │
│  Vote Keypair                                                              │
│  > ./vote-keypair.json                                                     │
│                                                                            │
│  Identity Keypair                                                          │
│  > ./validator-keypair.json                                                │
│                                                                            │
│  Withdraw Authority                                                        │
│  > ./withdraw-keypair.json                                                 │
│                                                                            │
│  Commission (%)                                                            │
│  > 5                                                                       │
│                                                                            │
│  Cost: ~0.0282 SOL                                                         │
│                                                                            │
├────────────────────────────────────────────────────────────────────────────┤
│ [Enter] submit  [Esc] back  [Tab] next field                              │
└────────────────────────────────────────────────────────────────────────────┘
```

**Key Features:**
- One field at a time focus
- Smart defaults pre-filled
- Minimal instructions

---

## Processing

```
┌─ SCILLA > Create Vote Account ─────────────────────────────────────────────┐
│ devnet │ 12.5 SOL │ Connected                                              │
├────────────────────────────────────────────────────────────────────────────┤
│                                                                            │
│                                                                            │
│                                                                            │
│                        Creating vote account...                            │
│                        [████████████░░░░] 75%                              │
│                                                                            │
│                        Confirming transaction                              │
│                                                                            │
│                                                                            │
│                                                                            │
├────────────────────────────────────────────────────────────────────────────┤
│ Processing...                                                              │
└────────────────────────────────────────────────────────────────────────────┘
```

---

## Success

```
┌─ SCILLA > Create Vote Account ─────────────────────────────────────────────┐
│ devnet │ 12.2 SOL │ Connected                                              │
├────────────────────────────────────────────────────────────────────────────┤
│                                                                            │
│  Success!                                                                  │
│                                                                            │
│  Vote Account                                                              │
│  GxN9rK2pB3vQ8fL5hJ4mW7sT1zC6yD2xE8uF9gH0iA                               │
│                                                                            │
│  Transaction                                                               │
│  5xK3j8fLmNp9qRs2tUv4wXy6zA1bC7dE3fG5hI8jK0lM                             │
│                                                                            │
│  What next?                                                                │
│  1  Delegate stake                                                         │
│  2  View details                                                           │
│  3  Back to menu                                                           │
│                                                                            │
├────────────────────────────────────────────────────────────────────────────┤
│ [1-3] select  [c] copy address  [Esc] back                                │
└────────────────────────────────────────────────────────────────────────────┘
```

**Key Features:**
- Numbered next actions
- Copy with single key `c`
- No extra noise

---

## Error

```
┌─ SCILLA > Create Vote Account ─────────────────────────────────────────────┐
│ devnet │ 12.5 SOL │ Connected                                              │
├────────────────────────────────────────────────────────────────────────────┤
│                                                                            │
│  Error: Insufficient funds                                                 │
│                                                                            │
│  Required: 0.0282 SOL                                                      │
│  Available: 0.0100 SOL                                                     │
│  Short: 0.0182 SOL                                                         │
│                                                                            │
│  1  Request airdrop                                                        │
│  2  Try again                                                              │
│  3  Back to menu                                                           │
│                                                                            │
├────────────────────────────────────────────────────────────────────────────┤
│ [1-3] select  [Esc] back                                                   │
└────────────────────────────────────────────────────────────────────────────┘
```

---

## Stake Delegation

```
┌─ SCILLA > Delegate Stake ──────────────────────────────────────────────────┐
│ devnet │ 12.5 SOL │ Connected                                              │
├────────────────────────────────────────────────────────────────────────────┤
│                                                                            │
│  Stake Account                                                             │
│  > ./stake-keypair.json                                                    │
│                                                                            │
│  Vote Account                                                              │
│  > GxN9rK2pB3vQ8fL5hJ4mW7sT1zC6yD2xE8uF9gH0iA                             │
│                                                                            │
│  Amount (SOL)                                                              │
│  > 10.0                                                                    │
│                                                                            │
│  Cost: ~0.00001 SOL                                                        │
│                                                                            │
├────────────────────────────────────────────────────────────────────────────┤
│ [Enter] submit  [Esc] back  [Tab] next field                              │
└────────────────────────────────────────────────────────────────────────────┘
```

---

## Validator Status

```
┌─ SCILLA > Validator Status ────────────────────────────────────────────────┐
│ devnet │ 12.5 SOL │ Connected                                              │
├────────────────────────────────────────────────────────────────────────────┤
│                                                                            │
│  Validator                                                                 │
│  GxN9rK2pB3vQ8fL5hJ4mW7sT1zC6yD2xE8uF9gH0iA                               │
│                                                                            │
│  Status          Active                                                    │
│  Commission      5%                                                        │
│  Epoch Credits   123,456                                                   │
│  Last Vote       12,345,678                                                │
│  Stake           1,234.5 SOL                                               │
│                                                                            │
│  1  Update commission                                                      │
│  2  Withdraw funds                                                         │
│  3  Deactivate                                                             │
│  4  Back to menu                                                           │
│                                                                            │
├────────────────────────────────────────────────────────────────────────────┤
│ [1-4] select  [r] refresh  [Esc] back                                      │
└────────────────────────────────────────────────────────────────────────────┘
```

---

## Account Balance

```
┌─ SCILLA > Account Balance ─────────────────────────────────────────────────┐
│ devnet │ Connected                                                         │
├────────────────────────────────────────────────────────────────────────────┤
│                                                                            │
│  Wallet Address                                                            │
│  > 7xM3j9fLmNp2qRs5tUv8wXy1zA4bC7dE0fG3hI6jK9lP                           │
│                                                                            │
│  Balance         12.5 SOL                                                  │
│  Rent Reserved   0.0014 SOL                                                │
│  Available       12.4986 SOL                                               │
│                                                                            │
│  1  Check another account                                                  │
│  2  Request airdrop                                                        │
│  3  Back to menu                                                           │
│                                                                            │
├────────────────────────────────────────────────────────────────────────────┤
│ [1-3] select  [r] refresh  [Esc] back                                      │
└────────────────────────────────────────────────────────────────────────────┘
```

---

## Switch Cluster

```
┌─ SCILLA > Switch Cluster ──────────────────────────────────────────────────┐
│ devnet │ 12.5 SOL │ Connected                                              │
├────────────────────────────────────────────────────────────────────────────┤
│                                                                            │
│  Current: devnet                                                           │
│                                                                            │
│  1  mainnet-beta                                                           │
│  2  devnet                                                                 │
│  3  testnet                                                                │
│  4  localnet                                                               │
│  5  Custom RPC                                                             │
│                                                                            │
├────────────────────────────────────────────────────────────────────────────┤
│ [1-5] select  [Esc] back                                                   │
└────────────────────────────────────────────────────────────────────────────┘
```

---

## Configuration

```
┌─ SCILLA > Configuration ───────────────────────────────────────────────────┐
│ devnet │ 12.5 SOL │ Connected                                              │
├────────────────────────────────────────────────────────────────────────────┤
│                                                                            │
│  1  Set default keypair                                                    │
│  2  Set RPC URL                                                            │
│  3  Set commitment level                                                   │
│  4  View all settings                                                      │
│  5  Reset to defaults                                                      │
│  6  Back to menu                                                           │
│                                                                            │
├────────────────────────────────────────────────────────────────────────────┤
│ [1-6] select  [Esc] back                                                   │
└────────────────────────────────────────────────────────────────────────────┘
```

---

## Help Overlay

```
┌─ SCILLA > Help ────────────────────────────────────────────────────────────┐
│                                                                            │
│  Navigation                                                                │
│  1-9       Select menu option                                              │
│  Enter     Submit form / confirm                                           │
│  Esc       Go back / cancel                                                │
│  Tab       Next field                                                      │
│  q         Quit application                                                │
│                                                                            │
│  Actions                                                                   │
│  c         Copy address/transaction                                        │
│  r         Refresh current view                                            │
│  h         Show this help                                                  │
│                                                                            │
│  Tips                                                                      │
│  - Press number keys without Enter                                         │
│  - Use Tab to move between form fields                                     │
│  - Esc always goes back one level                                          │
│                                                                            │
├────────────────────────────────────────────────────────────────────────────┤
│ [Esc] close                                                                │
└────────────────────────────────────────────────────────────────────────────┘
```

---

## Key Improvements Over Original

1. **Number Selection**: Press `1-9` for instant selection (no Enter needed)
2. **Single-Key Actions**: `c` copy, `r` refresh, `h` help, `q` quit
3. **Flat Hierarchy**: Maximum 2 levels deep - menu > action
4. **Minimal Visual Noise**: No emojis, icons, or decorative elements
5. **Consistent Layout**: Same structure every screen
6. **Smart Defaults**: Common values pre-filled
7. **Zero Cognitive Load**: Every option numbered, every key labeled
8. **Fast Navigation**: Esc always goes back, Tab moves forward

## Keyboard Map

```
Numbers (1-9)    Direct selection
Enter            Submit / Confirm
Esc              Back / Cancel
Tab              Next field
c                Copy
r                Refresh
h                Help
q                Quit
```

---

This design follows the lazygit philosophy: make the most common actions require the fewest keystrokes, use numbers for selection, and keep the interface clean and predictable.

