<script lang="ts">

    import {ScrollArea} from "$lib/components/ui/scroll-area";
    import {Button} from "$lib/components/ui/button";
    import * as AlertDialog from "$lib/components/ui/alert-dialog";
    import {onMount} from "svelte";
    import {listAccounts, removeAccount} from "$lib/api/api";
    import type {Account, PageParam} from "$lib/api/types";
    import {Plus, Pencil, Trash2, RefreshCw} from "@lucide/svelte";
    import AccountDialog from "$lib/components/account-dialog.svelte";
    
    let accounts: Account[] = $state([]);
    let currentPage = $state(1);
    let pageSize = $state(20);
    let total = $state(0);
    let loading = $state(false);
    let error = $state<string | null>(null);
    
    // Dialog state
    let dialogOpen = $state(false);
    let dialogMode = $state<"add" | "edit">("add");
    let selectedAccount = $state<Account | undefined>(undefined);
    
    // Delete confirmation dialog
    let deleteDialogOpen = $state(false);
    let accountToDelete = $state<string | null>(null);
    
    // Password state - in a real app, this should come from the unlock mechanism
    let password = $state("");
    
    async function loadAccounts() {
        if (!password) {
            error = "Password is required";
            return;
        }
        
        loading = true;
        error = null;
        try {
            const pageParam: PageParam = {
                current: currentPage,
                size: pageSize
            };
            const response = await listAccounts(pageParam, password);
            accounts = response.data;
            total = response.total;
            currentPage = response.current;
        } catch (err: any) {
            console.error("Failed to load accounts:", err);
            error = err.reason || "Failed to load accounts";
        } finally {
            loading = false;
        }
    }
    
    function openAddDialog() {
        dialogMode = "add";
        selectedAccount = undefined;
        dialogOpen = true;
    }
    
    function openEditDialog(account: Account) {
        dialogMode = "edit";
        selectedAccount = account;
        dialogOpen = true;
    }
    
    function confirmDelete(accountId: string) {
        accountToDelete = accountId;
        deleteDialogOpen = true;
    }
    
    async function handleDeleteAccount() {
        if (!password || !accountToDelete) {
            return;
        }
        
        try {
            await removeAccount(accountToDelete, password);
            deleteDialogOpen = false;
            accountToDelete = null;
            await loadAccounts();
        } catch (err: any) {
            console.error("Failed to delete account:", err);
            error = err.reason || "Failed to delete account";
            deleteDialogOpen = false;
        }
    }
    
    onMount(() => {
        // In a real implementation, get password from app state/unlock
        // For now, we'll just show a message
        console.log("Account list page loaded");
    });
</script>

<ScrollArea class="h-[var(--body-height)] w-full">
    <div class="relative flex flex-col overflow-hidden p-6">
        <div class="mb-4 flex items-center justify-between">
            <h1 class="text-2xl font-bold">Accounts</h1>
            <div class="flex gap-2">
                <Button size="sm" variant="outline" onclick={loadAccounts} disabled={loading || !password}>
                    <RefreshCw class="h-4 w-4 {loading ? 'animate-spin' : ''}" />
                    Refresh
                </Button>
                <Button size="sm" onclick={openAddDialog} disabled={!password}>
                    <Plus class="h-4 w-4 mr-1" />
                    Add Account
                </Button>
            </div>
        </div>
        
        {#if error}
            <div class="mb-4 rounded-md bg-destructive/10 p-4 text-destructive">
                {error}
            </div>
        {/if}
        
        {#if !password}
            <div class="mb-4 rounded-md bg-muted p-4">
                <p class="text-sm">Please unlock the app to view accounts</p>
            </div>
        {:else if loading}
            <div class="flex items-center justify-center p-8">
                <div class="text-muted-foreground">Loading accounts...</div>
            </div>
        {:else if accounts.length === 0}
            <div class="flex flex-col items-center justify-center p-8 text-center">
                <p class="mb-2 text-lg font-medium">No accounts yet</p>
                <p class="mb-4 text-sm text-muted-foreground">Add your first 2FA account to get started</p>
                <Button size="sm" onclick={openAddDialog}>
                    <Plus class="h-4 w-4 mr-1" />
                    Add Account
                </Button>
            </div>
        {:else}
            <div class="grid grid-cols-1 gap-4 md:grid-cols-2 lg:grid-cols-3 xl:grid-cols-4">
                {#each accounts as account (account.id)}
                    <div class="bg-card text-card-foreground flex flex-col rounded-lg border shadow-sm">
                        <div class="p-4">
                            <div class="mb-2 flex items-start justify-between">
                                <div class="flex-1">
                                    <h3 class="font-semibold">{account.issuer}</h3>
                                    <p class="text-sm text-muted-foreground">{account.label}</p>
                                </div>
                            </div>
                            
                            <div class="mb-3 flex items-center gap-2 text-sm text-muted-foreground">
                                <span class="rounded bg-muted px-2 py-1 text-xs">{account.type}</span>
                                <span class="rounded bg-muted px-2 py-1 text-xs">{account.algorithm}</span>
                                <span class="rounded bg-muted px-2 py-1 text-xs">{account.digits} digits</span>
                            </div>
                            
                            {#if account.note}
                                <p class="mb-3 text-xs text-muted-foreground">{account.note}</p>
                            {/if}
                            
                            <div class="flex gap-2">
                                <Button 
                                    size="sm" 
                                    variant="outline" 
                                    class="flex-1"
                                    onclick={() => openEditDialog(account)}
                                >
                                    <Pencil class="h-3 w-3" />
                                </Button>
                                <Button 
                                    size="sm" 
                                    variant="outline" 
                                    class="flex-1"
                                    onclick={() => confirmDelete(account.id)}
                                >
                                    <Trash2 class="h-3 w-3" />
                                </Button>
                            </div>
                        </div>
                    </div>
                {/each}
            </div>
            
            {#if total > pageSize}
                <div class="mt-6 flex items-center justify-center gap-2">
                    <Button 
                        size="sm" 
                        variant="outline"
                        disabled={currentPage === 1}
                        onclick={() => { currentPage--; loadAccounts(); }}
                    >
                        Previous
                    </Button>
                    <span class="text-sm text-muted-foreground">
                        Page {currentPage} of {Math.ceil(total / pageSize)}
                    </span>
                    <Button 
                        size="sm" 
                        variant="outline"
                        disabled={currentPage >= Math.ceil(total / pageSize)}
                        onclick={() => { currentPage++; loadAccounts(); }}
                    >
                        Next
                    </Button>
                </div>
            {/if}
        {/if}
    </div>
</ScrollArea>

<AccountDialog 
    bind:open={dialogOpen}
    mode={dialogMode}
    account={selectedAccount}
    password={password}
    onSuccess={loadAccounts}
/>

<AlertDialog.Root bind:open={deleteDialogOpen}>
    <AlertDialog.Content>
        <AlertDialog.Header>
            <AlertDialog.Title>Are you sure?</AlertDialog.Title>
            <AlertDialog.Description>
                This action cannot be undone. This will permanently delete the account from your database.
            </AlertDialog.Description>
        </AlertDialog.Header>
        <AlertDialog.Footer>
            <AlertDialog.Cancel>Cancel</AlertDialog.Cancel>
            <AlertDialog.Action onclick={handleDeleteAccount}>Delete</AlertDialog.Action>
        </AlertDialog.Footer>
    </AlertDialog.Content>
</AlertDialog.Root>