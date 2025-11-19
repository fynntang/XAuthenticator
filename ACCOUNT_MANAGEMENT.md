# Account Management Implementation

This document describes the new account management functionality implemented in XAuthenticator.

## Overview

The account management system provides a 1Password-style UI for managing TOTP (Time-based One-Time Password) accounts. All account data is securely stored in an encrypted KDBX (KeePass) database.

## Features

### Backend (Rust)

The backend provides the following API commands:

- **`list_accounts()`** - Retrieve all accounts from the database
- **`create_account(request)`** - Add a new TOTP account
- **`update_account(request)`** - Modify an existing account
- **`delete_account(account_id)`** - Remove an account
- **`get_code(account_id)`** - Generate the current TOTP code for an account

#### Account Data Structure

```rust
pub struct Account {
    pub id: String,              // UUID
    pub name: String,            // Account name (e.g., "GitHub")
    pub issuer: Option<String>,  // Service provider
    pub account_name: Option<String>,  // Username/email
    pub secret: String,          // Base32-encoded TOTP secret
    pub algorithm: String,       // SHA1, SHA256, or SHA512
    pub digits: u32,             // Usually 6 or 8
    pub period: u32,             // Refresh period in seconds (usually 30)
    pub icon: Option<String>,    // Icon URL or identifier
    pub notes: Option<String>,   // User notes
    pub created_at: i64,         // Creation timestamp
    pub modified_at: i64,        // Last modification timestamp
}
```

### Frontend (Svelte)

The frontend provides an intuitive UI with the following components:

#### AccountList Component

The main component that displays all accounts in a responsive grid layout:

- **Card-based Design**: Each account is shown in a card with:
  - Account name and metadata (issuer, account name)
  - Live TOTP code formatted as groups of 3 digits
  - Visual countdown timer with progress bar
  - Edit and delete buttons
  - Copy-to-clipboard button

- **Auto-refresh**: TOTP codes automatically update every second, with visual countdown

- **Color-coded Progress**:
  - Normal: Blue progress bar
  - Warning (≤5s): Yellow/warning color
  - Critical (≤3s): Red/destructive color

#### AccountDialog Component

Dialog for creating and editing accounts:

- **Required Fields**:
  - Name: Display name for the account
  - Secret: Base32-encoded TOTP secret

- **Optional Fields**:
  - Issuer: Service provider name
  - Account Name: Username or email
  - Algorithm: SHA1 (default), SHA256, or SHA512
  - Digits: Number of digits in code (6 or 8)
  - Period: Refresh interval in seconds (15-60)
  - Notes: Free-form text notes

#### DeleteConfirmDialog Component

Simple confirmation dialog for account deletion with:
- Account name display
- Warning about irreversible action
- Cancel and confirm buttons

## Security

All account data is stored in an encrypted KDBX4 database:

- Database is encrypted with user's master password
- Password is stored in memory only while the app is unlocked
- Database and password are cleared from memory when the app is locked
- TOTP secrets are protected by KDBX encryption at rest

## Usage

### Adding an Account

1. Click the "Add Account" button
2. Enter the account name and TOTP secret (required)
3. Optionally fill in additional fields
4. Click "Add Account" to save

### Editing an Account

1. Click the edit icon on an account card
2. Modify the desired fields
3. Click "Update Account" to save changes

### Deleting an Account

1. Click the delete icon on an account card
2. Confirm the deletion in the dialog
3. The account is permanently removed from the database

### Copying TOTP Code

1. Click the copy icon next to a TOTP code
2. The code is copied to your clipboard
3. A toast notification confirms the action

## Technical Details

### TOTP Code Generation

The system supports multiple TOTP algorithms:
- **SHA1**: Most common, default algorithm
- **SHA256**: Enhanced security
- **SHA512**: Maximum security

Codes are generated using the `totp-lite` crate with proper timing alignment.

### Database Operations

All database operations follow this pattern:
1. Check if app is locked (return error if locked)
2. Verify database is loaded in memory
3. Perform the operation on the in-memory database
4. Save changes back to the KDBX file
5. Update modification timestamps

### State Management

The AppState stores:
- `db`: Optional Database (loaded when unlocked)
- `master_password`: Optional String (stored when unlocked)
- `is_locked`: Boolean flag
- `locked_timestamp`: When the app was locked
- `config`: Application configuration

## Future Enhancements

Potential improvements:
- QR code scanner for easier account setup
- Search and filter functionality
- Account grouping/categories
- Backup and restore functionality
- Import from other authenticator apps
- Export accounts
- Bulk operations
