<script lang="ts">
  import { onMount } from "svelte";
  import { listAccounts, removeAccount } from "$lib/api/api";
  import type { Account } from "$lib/api/types";
  import { Button } from "$lib/components/ui/button";
  import * as Card from "$lib/components/ui/card";
  import * as AlertDialog from "$lib/components/ui/alert-dialog";
  import { Plus, Trash2, Pencil } from "@lucide/svelte";
  import { toast } from "svelte-sonner";
  import AddAccountDialog from "./AddAccountDialog.svelte";
  import EditAccountDialog from "./EditAccountDialog.svelte";

  let accounts: Account[] = [];
  let total = 0;
  let currentPage = 0;
  let pageSize = 16;
  let loading = false;
  let showAddDialog = false;
  let showEditDialog = false;
  let showDeleteDialog = false;
  let accountToDelete: Account | null = null;
  let accountToEdit: Account | null = null;

  async function loadAccounts() {
    loading = true;
    try {
      const response = await listAccounts(currentPage, pageSize);
      accounts = response.data;
      total = response.total;
    } catch (error: any) {
      toast.error("Failed to load accounts", {
        description: error.reason || "Unknown error",
      });
    } finally {
      loading = false;
    }
  }

  async function handleDelete() {
    if (!accountToDelete) return;
    
    try {
      await removeAccount(accountToDelete.id);
      toast.success("Account deleted successfully");
      accountToDelete = null;
      showDeleteDialog = false;
      await loadAccounts();
    } catch (error: any) {
      toast.error("Failed to delete account", {
        description: error.reason || "Unknown error",
      });
    }
  }

  function handleEdit(account: Account) {
    accountToEdit = account;
    showEditDialog = true;
  }

  function handleAddSuccess() {
    showAddDialog = false;
    loadAccounts();
  }

  function handleEditSuccess() {
    showEditDialog = false;
    accountToEdit = null;
    loadAccounts();
  }

  function confirmDelete(account: Account) {
    accountToDelete = account;
    showDeleteDialog = true;
  }

  function cancelDelete() {
    accountToDelete = null;
    showDeleteDialog = false;
  }

  onMount(() => {
    loadAccounts();
  });
</script>

<div class="flex flex-col gap-4 h-full">
  <div class="flex justify-between items-center">
    <h1 class="text-2xl font-bold">Accounts</h1>
    <Button onclick={() => (showAddDialog = true)}>
      <Plus class="mr-2 h-4 w-4" />
      Add Account
    </Button>
  </div>

  {#if loading}
    <div class="text-center py-8">Loading accounts...</div>
  {:else if accounts.length === 0}
    <div class="text-center py-8 text-muted-foreground">
      No accounts found. Add your first account to get started.
    </div>
  {:else}
    <div class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 xl:grid-cols-4 gap-4">
      {#each accounts as account (account.id)}
        <Card.Root>
          <Card.Header>
            <Card.Title class="text-lg">{account.issuer}</Card.Title>
            <Card.Description>{account.label}</Card.Description>
          </Card.Header>
          <Card.Content>
            <div class="space-y-2 text-sm">
              <div class="flex justify-between">
                <span class="text-muted-foreground">Type:</span>
                <span class="font-medium">{account.type}</span>
              </div>
              <div class="flex justify-between">
                <span class="text-muted-foreground">Algorithm:</span>
                <span class="font-medium">{account.algorithm}</span>
              </div>
              <div class="flex justify-between">
                <span class="text-muted-foreground">Digits:</span>
                <span class="font-medium">{account.digits}</span>
              </div>
              {#if account.period}
                <div class="flex justify-between">
                  <span class="text-muted-foreground">Period:</span>
                  <span class="font-medium">{account.period}s</span>
                </div>
              {/if}
            </div>
          </Card.Content>
          <Card.Footer class="flex gap-2">
            <Button
              variant="outline"
              size="sm"
              class="flex-1"
              onclick={() => handleEdit(account)}
            >
              <Pencil class="h-4 w-4 mr-2" />
              Edit
            </Button>
            <Button
              variant="destructive"
              size="sm"
              onclick={() => confirmDelete(account)}
            >
              <Trash2 class="h-4 w-4" />
            </Button>
          </Card.Footer>
        </Card.Root>
      {/each}
    </div>
  {/if}

  <div class="text-center text-sm text-muted-foreground mt-4">
    Total accounts: {total}
  </div>
</div>

<AddAccountDialog bind:open={showAddDialog} on:success={handleAddSuccess} />

{#if accountToEdit}
  <EditAccountDialog
    bind:open={showEditDialog}
    account={accountToEdit}
    on:success={handleEditSuccess}
  />
{/if}

<AlertDialog.Root bind:open={showDeleteDialog}>
  <AlertDialog.Content>
    <AlertDialog.Header>
      <AlertDialog.Title>Are you sure?</AlertDialog.Title>
      <AlertDialog.Description>
        This action cannot be undone. This will permanently delete the account
        "{accountToDelete?.issuer} - {accountToDelete?.label}".
      </AlertDialog.Description>
    </AlertDialog.Header>
    <AlertDialog.Footer>
      <AlertDialog.Cancel onclick={cancelDelete}>
        Cancel
      </AlertDialog.Cancel>
      <AlertDialog.Action onclick={handleDelete}>Delete</AlertDialog.Action>
    </AlertDialog.Footer>
  </AlertDialog.Content>
</AlertDialog.Root>
