# KDBX Account CRUD Implementation - Testing Guide

## Overview
This document describes the implementation of CRUD (Create, Read, Update, Delete) operations for KDBX accounts in XAuthenticator.

## Implementation Summary

### Backend Changes (Rust)

#### 1. Entity Types (`crates/entity/src/lib.rs`)
- **CreateAccountRequest**: Request type for creating new accounts
  - Fields: issuer, label, type, algorithm, digits, period, counter, secret, icon, note
- **UpdateAccountRequest**: Request type for updating existing accounts
  - Fields: id, issuer (optional), label (optional), icon (optional), note (optional)
- **PagedResponse<T>**: Generic paged response type
  - Fields: data, total, current, size
- **response::Account**: Enhanced with icon, note, createdAt, updatedAt fields

#### 2. KDBX Database Operations (`src-tauri/src/commands/mod.rs`)

**Helper Functions:**
- `open_database()`: Opens KDBX database with password validation
- `save_database()`: Saves KDBX database with encryption
- `entry_to_account()`: Converts keepass Entry to response Account

**CRUD Commands:**
- `list_accounts(page_param, password)`: Retrieve paginated list of accounts
  - Returns: PagedResponse<Account>
  - Recursively searches all groups
  - Supports pagination
  
- `add_account(request, password)`: Add new account
  - Returns: String (account ID)
  - Validates required fields (issuer, secret)
  - Stores OTP metadata as tags
  - Uses UUID v7 for IDs
  
- `update_account(request, password)`: Update existing account
  - Returns: void
  - Updates issuer, label, and note
  - Finds account recursively in groups
  
- `remove_account(account_id, password)`: Delete account
  - Returns: void
  - Validates UUID format
  - Removes entry from database

**Security Features:**
- All operations require password
- App lock state checked before operations
- Passwords validated against strength requirements
- Database encryption using keepass

#### 3. Dependencies Added
- `base64 = "0.22"` in src-tauri/Cargo.toml

### Frontend Changes (TypeScript/Svelte)

#### 1. Type Definitions (`src/lib/api/types.ts`)
Extended Account type with:
- algorithm, digits, period, counter
- secretCipher, icon, note
- createdAt, updatedAt

Added request types:
- CreateAccountRequest
- UpdateAccountRequest
- PagedResponse<T>

#### 2. API Methods (`src/lib/api/api.ts`)
- `listAccounts(pageParam, password)`: Fetch paginated accounts
- `addAccount(request, password)`: Create new account
- `updateAccount(request, password)`: Update existing account
- `removeAccount(accountId, password)`: Delete account

#### 3. Account Dialog Component (`src/lib/components/account-dialog.svelte`)
Reusable dialog for add/edit operations:
- **Add Mode**: Full form with all OTP settings
  - Issuer, label, secret (required)
  - Type (TOTP/HOTP), algorithm (SHA1/SHA256/SHA512)
  - Digits, period/counter, note
- **Edit Mode**: Limited form for metadata updates
  - Issuer, label, note only
  - Cannot change OTP settings
- Form validation
- Error handling
- Loading states

#### 4. Account List Page (`src/routes/(app)/+page.svelte`)
Complete account management UI:
- Responsive grid layout (1-4 columns based on screen size)
- Pagination controls
- Account cards showing:
  - Issuer and label
  - Type, algorithm, digits badges
  - Optional note
  - Edit and delete buttons
- Add account button
- Refresh functionality
- Delete confirmation dialog (AlertDialog)
- Empty state with call-to-action
- Error display
- Loading states

## Testing Checklist

### Manual Testing Steps

#### 1. Test Account Creation
- [ ] Open the app and unlock with password
- [ ] Click "Add Account" button
- [ ] Fill in required fields:
  - Issuer: "GitHub"
  - Label: "user@example.com"
  - Secret: Valid base32 secret
  - Type: TOTP
  - Algorithm: SHA1
  - Digits: 6
  - Period: 30
- [ ] Click "Save"
- [ ] Verify account appears in list
- [ ] Verify account is saved in KDBX file

#### 2. Test Account Listing
- [ ] Create multiple accounts (at least 3)
- [ ] Verify all accounts display correctly
- [ ] Check pagination works (if > 20 accounts)
- [ ] Verify account details shown correctly:
  - Issuer name
  - Account label
  - Type badge
  - Algorithm badge
  - Digits badge
  - Note (if present)

#### 3. Test Account Update
- [ ] Click edit button on an account
- [ ] Modify issuer name
- [ ] Modify label
- [ ] Add/modify note
- [ ] Click "Save"
- [ ] Verify changes reflected in list
- [ ] Verify changes persisted in KDBX

#### 4. Test Account Deletion
- [ ] Click delete button on an account
- [ ] Verify confirmation dialog appears
- [ ] Click "Cancel" - verify account not deleted
- [ ] Click delete again
- [ ] Click "Delete" in confirmation
- [ ] Verify account removed from list
- [ ] Verify account removed from KDBX

#### 5. Test Pagination
- [ ] Create 25+ accounts
- [ ] Verify page 1 shows first 20 accounts
- [ ] Click "Next" button
- [ ] Verify page 2 shows remaining accounts
- [ ] Click "Previous" button
- [ ] Verify navigation works correctly

#### 6. Test Security
- [ ] Lock the app
- [ ] Try to view accounts - verify blocked
- [ ] Try to add account - verify blocked
- [ ] Unlock app with password
- [ ] Verify operations work again

#### 7. Test Error Handling
- [ ] Try to add account without issuer - verify error
- [ ] Try to add account without secret - verify error
- [ ] Try to add account with wrong password - verify error
- [ ] Try to edit non-existent account - verify error
- [ ] Try to delete non-existent account - verify error

### Expected Behavior

**Add Account:**
- Creates new entry in KDBX database
- Generates UUID v7 for account ID
- Stores secret encrypted
- Stores OTP metadata as tags
- Returns account ID on success

**List Accounts:**
- Retrieves all entries from all groups
- Converts entries to Account type
- Applies pagination
- Returns total count and current page

**Update Account:**
- Finds account by UUID
- Updates allowed fields only
- Preserves secret and OTP settings
- Saves to KDBX

**Delete Account:**
- Finds and removes entry by UUID
- Updates KDBX file
- Permanently deletes account

## Known Limitations

1. **Password Required**: Every operation requires password input (not cached)
   - In production, password should be cached in app state after unlock
   
2. **Secret Update**: Cannot update the secret after account creation
   - Would require delete and recreate
   
3. **OTP Settings**: Cannot modify type, algorithm, digits, period, or counter after creation
   - Edit mode only allows issuer, label, and note updates
   
4. **Icon Support**: Icon field exists but not fully implemented
   - Would require image upload/selection UI
   
5. **Group Management**: All accounts stored in root group
   - No support for organizing accounts into groups yet

## Future Enhancements

1. **Password Management**:
   - Cache password in encrypted app state after unlock
   - Auto-lock after timeout
   - Biometric authentication support

2. **Advanced Features**:
   - QR code scanning for easy account addition
   - Backup/restore functionality
   - Account search and filtering
   - Bulk operations
   - Account organization with groups
   - Icon/logo support for accounts
   - OTP code generation and display

3. **UI/UX Improvements**:
   - Drag-and-drop reordering
   - Keyboard shortcuts
   - Better mobile responsiveness
   - Dark mode refinements

## Security Considerations

1. **Encryption**: All secrets stored encrypted in KDBX
2. **Authentication**: Password required for all operations
3. **Lock State**: App lock state checked before operations
4. **Password Strength**: Validation enforced on initialization
5. **No Plaintext**: Secrets never stored in plaintext
6. **Database Integrity**: KDBX format ensures data integrity

## Troubleshooting

**Issue**: "Password is required" error
- **Solution**: Ensure password is provided and app is unlocked

**Issue**: "Account not found" error
- **Solution**: Verify account ID is valid and account exists

**Issue**: "Failed to save database" error
- **Solution**: Check file permissions and disk space

**Issue**: Accounts not appearing after creation
- **Solution**: Click refresh button or reload page

**Issue**: Cannot edit secret or OTP settings
- **Solution**: This is by design - delete and recreate account if needed
