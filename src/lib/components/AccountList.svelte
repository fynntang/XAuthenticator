<script lang="ts">
    import type {Account} from "$lib/api/types";
    import {createAccount, deleteAccount, getCode, listAccounts, updateAccount} from "$lib/api/api";
    import {onMount} from "svelte";
    import {Button} from "$lib/components/ui/button";
    import {Plus, RefreshCw, Settings, Trash2, Copy, Edit} from '@lucide/svelte';
    import {toast} from 'svelte-sonner';
    import AccountDialog from './AccountDialog.svelte';
    import DeleteConfirmDialog from './DeleteConfirmDialog.svelte';
    
    let accounts: Account[] = [];
    let loading = false;
    let showAddDialog = false;
    let showEditDialog = false;
    let showDeleteDialog = false;
    let selectedAccount: Account | null = null;
    let totpCodes: Map<string, { code: string, remaining: number }> = new Map();
    
    async function loadAccounts() {
        try {
            loading = true;
            accounts = await listAccounts();
            // Generate TOTP codes for all accounts
            for (const account of accounts) {
                await updateTOTPCode(account.id);
            }
        } catch (error: any) {
            toast.error('Failed to load accounts: ' + (error.reason || error.message || 'Unknown error'));
        } finally {
            loading = false;
        }
    }
    
    async function updateTOTPCode(accountId: string) {
        try {
            const code = await getCode(accountId);
            const now = Math.floor(Date.now() / 1000);
            const account = accounts.find(a => a.id === accountId);
            if (account) {
                const remaining = account.period - (now % account.period);
                totpCodes.set(accountId, { code, remaining });
                totpCodes = totpCodes; // trigger reactivity
            }
        } catch (error: any) {
            console.error('Failed to generate TOTP code:', error);
        }
    }
    
    async function handleSaveAccount(account: Partial<Account>) {
        try {
            if (selectedAccount) {
                // Update existing account
                await updateAccount({
                    id: selectedAccount.id,
                    ...account
                });
                toast.success('Account updated successfully');
            } else {
                // Create new account
                await createAccount({
                    name: account.name!,
                    secret: account.secret!,
                    issuer: account.issuer,
                    accountName: account.accountName,
                    algorithm: account.algorithm,
                    digits: account.digits,
                    period: account.period,
                    icon: account.icon,
                    notes: account.notes
                });
                toast.success('Account created successfully');
            }
            await loadAccounts();
            showAddDialog = false;
            showEditDialog = false;
            selectedAccount = null;
        } catch (error: any) {
            toast.error('Failed to save account: ' + (error.reason || error.message || 'Unknown error'));
        }
    }
    
    async function handleDeleteAccount() {
        if (selectedAccount) {
            try {
                await deleteAccount(selectedAccount.id);
                toast.success('Account deleted successfully');
                await loadAccounts();
                showDeleteDialog = false;
                selectedAccount = null;
            } catch (error: any) {
                toast.error('Failed to delete account: ' + (error.reason || error.message || 'Unknown error'));
            }
        }
    }
    
    function copyToClipboard(text: string) {
        navigator.clipboard.writeText(text).then(() => {
            toast.success('Copied to clipboard');
        }).catch(() => {
            toast.error('Failed to copy to clipboard');
        });
    }
    
    function formatCode(code: string, digits: number): string {
        // Format code as groups of 3 digits separated by spaces
        const groups = [];
        for (let i = 0; i < code.length; i += 3) {
            groups.push(code.substring(i, Math.min(i + 3, code.length)));
        }
        return groups.join(' ');
    }
    
    onMount(() => {
        loadAccounts();
        
        // Update TOTP codes every second
        const interval = setInterval(() => {
            const now = Math.floor(Date.now() / 1000);
            for (const account of accounts) {
                const currentCode = totpCodes.get(account.id);
                if (currentCode) {
                    const remaining = account.period - (now % account.period);
                    if (remaining !== currentCode.remaining) {
                        if (remaining >= account.period - 1) {
                            // Time to refresh the code
                            updateTOTPCode(account.id);
                        } else {
                            totpCodes.set(account.id, { ...currentCode, remaining });
                            totpCodes = totpCodes; // trigger reactivity
                        }
                    }
                }
            }
        }, 1000);
        
        return () => clearInterval(interval);
    });
</script>

<div class="h-full flex flex-col">
    <div class="flex items-center justify-between p-6 border-b">
        <h1 class="text-2xl font-semibold">Accounts</h1>
        <div class="flex gap-2">
            <Button variant="outline" size="icon" onclick={loadAccounts} disabled={loading}>
                {#if loading}
                    <RefreshCw class="h-4 w-4 animate-spin" />
                {:else}
                    <RefreshCw class="h-4 w-4" />
                {/if}
            </Button>
            <Button onclick={() => { showAddDialog = true; selectedAccount = null; }}>
                <Plus class="h-4 w-4 mr-2" />
                Add Account
            </Button>
        </div>
    </div>
    
    <div class="flex-1 overflow-auto p-6">
        {#if loading && accounts.length === 0}
            <div class="flex items-center justify-center h-full">
                <p class="text-muted-foreground">Loading accounts...</p>
            </div>
        {:else if accounts.length === 0}
            <div class="flex flex-col items-center justify-center h-full gap-4">
                <p class="text-muted-foreground">No accounts yet</p>
                <Button onclick={() => { showAddDialog = true; selectedAccount = null; }}>
                    <Plus class="h-4 w-4 mr-2" />
                    Add Your First Account
                </Button>
            </div>
        {:else}
            <div class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 xl:grid-cols-4 gap-4">
                {#each accounts as account (account.id)}
                    {@const totpInfo = totpCodes.get(account.id)}
                    <div class="bg-card text-card-foreground rounded-lg border shadow-sm hover:shadow-md transition-shadow">
                        <div class="p-4">
                            <div class="flex items-start justify-between mb-3">
                                <div class="flex-1">
                                    <h3 class="font-semibold text-lg">{account.name}</h3>
                                    {#if account.issuer}
                                        <p class="text-sm text-muted-foreground">{account.issuer}</p>
                                    {/if}
                                    {#if account.accountName}
                                        <p class="text-xs text-muted-foreground">{account.accountName}</p>
                                    {/if}
                                </div>
                                <div class="flex gap-1">
                                    <Button
                                        variant="ghost"
                                        size="icon"
                                        class="h-8 w-8"
                                        onclick={() => { selectedAccount = account; showEditDialog = true; }}
                                    >
                                        <Edit class="h-4 w-4" />
                                    </Button>
                                    <Button
                                        variant="ghost"
                                        size="icon"
                                        class="h-8 w-8 text-destructive"
                                        onclick={() => { selectedAccount = account; showDeleteDialog = true; }}
                                    >
                                        <Trash2 class="h-4 w-4" />
                                    </Button>
                                </div>
                            </div>
                            
                            {#if totpInfo}
                                <div class="mb-3">
                                    <div class="flex items-center justify-between mb-1">
                                        <span class="font-mono text-3xl tracking-wider">
                                            {formatCode(totpInfo.code, account.digits)}
                                        </span>
                                        <Button
                                            variant="ghost"
                                            size="icon"
                                            class="h-8 w-8"
                                            onclick={() => copyToClipboard(totpInfo.code)}
                                        >
                                            <Copy class="h-4 w-4" />
                                        </Button>
                                    </div>
                                    <div class="w-full bg-muted rounded-full h-1.5">
                                        <div
                                            class="bg-primary h-1.5 rounded-full transition-all"
                                            class:bg-warning={totpInfo.remaining <= 5}
                                            class:bg-destructive={totpInfo.remaining <= 3}
                                            style="width: {(totpInfo.remaining / account.period) * 100}%"
                                        ></div>
                                    </div>
                                    <p class="text-xs text-muted-foreground mt-1 text-right">
                                        {totpInfo.remaining}s
                                    </p>
                                </div>
                            {/if}
                            
                            {#if account.notes}
                                <p class="text-sm text-muted-foreground line-clamp-2">{account.notes}</p>
                            {/if}
                        </div>
                    </div>
                {/each}
            </div>
        {/if}
    </div>
</div>

<AccountDialog
    bind:open={showAddDialog}
    account={null}
    on:save={(e) => handleSaveAccount(e.detail)}
/>

<AccountDialog
    bind:open={showEditDialog}
    account={selectedAccount}
    on:save={(e) => handleSaveAccount(e.detail)}
/>

<DeleteConfirmDialog
    bind:open={showDeleteDialog}
    accountName={selectedAccount?.name || ''}
    on:confirm={handleDeleteAccount}
/>

<style>
    .bg-warning {
        background-color: hsl(var(--warning));
    }
</style>
